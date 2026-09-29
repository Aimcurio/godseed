use std::fs;
use std::io::Cursor;
use godseed_core::{
    components::{CitizenMeta, Demographics, NpcSchedule, OccupationProfile, PhysicalNeeds, PlayerMarker},
    content::caps,
    persistence::{
        load_snapshot, save_snapshot, save_snapshot_v1, CitizenSnapshotV1,
        SimulationSnapshotV1, FORMAT_VERSION_V1, FORMAT_VERSION_V2, MAGIC_V2,
    },
    resources::{DocumentRegistry, PendingConsequenceRegistry, ReturnDigestLog},
    sim::Simulation,
    types::{
        CitizenId, ConsequenceStage, ConsequenceType, DocumentType,
        InscribedDocument, LocationId, OccupationType, PlayerAction,
        ResourceType, TalkTopic, TriggerCondition,
    },
};

// ── 1. Causal Continuity Across Save/Load (Section 11) ──────────────────────────

#[test]
fn test_causal_continuity_across_save_load() {
    let mut sim = Simulation::new();

    // Move to West Woods (loc 11)
    sim.push_action(PlayerAction::Move { to: LocationId(7) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(8) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(11) });
    sim.step();
    sim.drain_results();

    while sim.summary().hour < 8 {
        sim.push_action(PlayerAction::Wait { ticks: 1 });
        sim.step();
        sim.drain_results();
    }

    // Perform HelpWithFelling with Tomas Birch (Citizen 6)
    sim.push_action(PlayerAction::HelpWithFelling { npc: CitizenId(6) });
    sim.step();
    let res = sim.drain_results();
    assert!(res[0].success);

    // Verify consequence registered as Active
    {
        let reg = sim.world.resource::<PendingConsequenceRegistry>();
        assert_eq!(reg.consequences.len(), 1);
        assert_eq!(reg.consequences[0].stage, ConsequenceStage::Active);
    }

    // Move back to Inn
    sim.push_action(PlayerAction::Move { to: LocationId(9) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(1) });
    sim.step();
    sim.drain_results();

    // Save simulation to file
    let save_path = "saves/test_causal_continuity.gs2";
    fs::create_dir_all("saves").unwrap();
    sim.save(save_path).expect("Save must succeed");

    // Drop original simulation and reload
    drop(sim);
    let mut loaded_sim = Simulation::load_from_file(save_path).expect("Load must succeed");

    // Verify consequence survived save/load intact
    {
        let reg = loaded_sim.world.resource::<PendingConsequenceRegistry>();
        assert_eq!(reg.consequences.len(), 1);
        assert_eq!(reg.consequences[0].stage, ConsequenceStage::Active);
    }

    // Advance 720 ticks (30 in-game days) with provisions
    for _ in 0..720 {
        loaded_sim.step();
        let mut q = loaded_sim.world.query_filtered::<(&mut PhysicalNeeds, &mut Demographics), bevy_ecs::prelude::With<PlayerMarker>>();
        for (mut needs, mut demo) in q.iter_mut(&mut loaded_sim.world) {
            needs.satiety = 100;
            demo.health = 100;
        }
    }

    // Verify consequence transitioned to Matured after reload and 30-day fast-forward
    {
        let reg = loaded_sim.world.resource::<PendingConsequenceRegistry>();
        assert_eq!(reg.consequences[0].stage, ConsequenceStage::Matured);
    }

    // Verify Runn Birch is now an Artisan working at Forge (Location 2)
    {
        let mut query = loaded_sim.world.query::<(&CitizenMeta, &NpcSchedule, &OccupationProfile)>();
        let mut found_runn = false;
        for (meta, sched, occ) in query.iter(&loaded_sim.world) {
            if meta.id == CitizenId(12) {
                found_runn = true;
                assert_eq!(occ.occupation, OccupationType::Artisan);
                assert_eq!(sched.work_location, LocationId(2));
            }
        }
        assert!(found_runn);
    }

    // Verify ReturnDigestLog contains digest for Mira
    {
        let digest_log = loaded_sim.world.resource::<ReturnDigestLog>();
        assert!(digest_log.find_digest_for(CitizenId(1)).is_some(), "Mira digest must exist");
    }

    // Wait until hour 14 when Mira is behind the bar
    while loaded_sim.summary().hour != 14 {
        loaded_sim.push_action(PlayerAction::Wait { ticks: 1 });
        loaded_sim.step();
        loaded_sim.drain_results();
    }

    loaded_sim.push_action(PlayerAction::Talk {
        npc: CitizenId(1),
        topic: TalkTopic::Greeting,
    });
    loaded_sim.step();
    let mira_res = loaded_sim.drain_results();
    assert!(mira_res[0].success);
    assert!(
        mira_res[0].message.contains("You've been gone a spell")
            && mira_res[0].message.contains("Tomas works alone now"),
        "Mira must deliver epistemic return digest: {}",
        mira_res[0].message
    );

    let _ = fs::remove_file(save_path);
}

// ── 2. Proof A Save/Load Continuity (Section 12) ────────────────────────────────

#[test]
fn test_proof_a_save_load_continuity() {
    let mut sim = Simulation::new();

    // Advance 24 ticks (1 day)
    sim.advance(24);

    // Move to Forest Edge (loc 11) where Tomas works
    sim.push_action(PlayerAction::Move { to: LocationId(7) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(8) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(11) });
    sim.step();
    sim.drain_results();

    while sim.summary().hour < 8 {
        sim.push_action(PlayerAction::Wait { ticks: 1 });
        sim.step();
        sim.drain_results();
    }

    // Perform HelpWithFelling with Tomas Birch (Citizen 6)
    sim.push_action(PlayerAction::HelpWithFelling { npc: CitizenId(6) });
    sim.step();
    let tomas_res = sim.drain_results();
    assert!(tomas_res[0].success);

    // Verify Tomas recorded episodic memory anchor and relational delta
    let anchor_count_before = {
        let mut q = sim.world.query::<(&CitizenMeta, &godseed_core::components::EpisodicMemory)>();
        let (_, mem) = q.iter(&sim.world).find(|(m, _)| m.id == CitizenId(6)).unwrap();
        mem.anchors.len()
    };
    assert!(anchor_count_before >= 1, "Tomas must have an episodic memory anchor");

    // Save simulation
    let save_path = "saves/test_proof_a_save.gs2";
    fs::create_dir_all("saves").unwrap();
    sim.save(save_path).expect("Save must succeed");

    // Drop and reload
    drop(sim);
    let mut loaded_sim = Simulation::load_from_file(save_path).expect("Load must succeed");

    // Verify anchor and relational delta survived intact in loaded sim
    {
        let mut q = loaded_sim.world.query::<(&CitizenMeta, &godseed_core::components::EpisodicMemory, &godseed_core::components::RelationalLedger)>();
        let (_, mem, rel) = q.iter(&loaded_sim.world).find(|(m, _, _)| m.id == CitizenId(6)).unwrap();
        assert_eq!(mem.anchors.len(), anchor_count_before, "Anchor count must survive save/load");
        let bond = rel.get_bond(CitizenId::PLAYER);
        assert!(bond.sentiment > 40 && bond.trust > 40, "Tomas relational bond toward player must survive save/load");
    }

    // Greet Tomas again: memory-aware response
    loaded_sim.push_action(PlayerAction::Talk {
        npc: CitizenId(6),
        topic: TalkTopic::Greeting,
    });
    loaded_sim.step();
    let second_res = loaded_sim.drain_results();
    assert!(second_res[0].success);
    assert!(
        second_res[0].message.contains("oak we brought down together"),
        "Tomas must acknowledge joint felling upon greeting: {}",
        second_res[0].message
    );

    let _ = fs::remove_file(save_path);
}

// ── 3. Proof B Save/Load Continuity (Section 13) ────────────────────────────────

#[test]
fn test_proof_b_save_load_continuity() {
    let mut sim = Simulation::new();

    // 1. Move to Old Archive (loc 8)
    sim.push_action(PlayerAction::Move { to: LocationId(7) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(8) });
    sim.step();

    // Study archive
    sim.push_action(PlayerAction::StudyArchive);
    sim.step();

    // Practice Inscription
    sim.push_action(PlayerAction::Practice { capability: caps::INSCRIPTION });
    sim.step();
    sim.drain_results();

    // Advance 30 ticks to trigger Stage 1
    sim.advance(30);

    // Inscribe 5 observations
    for i in 1..=5 {
        sim.push_action(PlayerAction::Inscribe {
            observation: format!("Field observation #{}: Inscribing communal dynamics in Thornveil.", i),
        });
        sim.step();
    }
    sim.drain_results();

    // Set relationship with Elder Voss > 60
    {
        use godseed_core::resources::RelationshipLedger;
        let mut rels = sim.world.resource_mut::<RelationshipLedger>();
        rels.set(CitizenId::PLAYER, CitizenId(5), 75);
    }

    // Advance 30 ticks to trigger Stage 2 (The Settlement Chronicler)
    sim.advance(30);
    assert_eq!(sim.summary().player.unwrap().transformation_stage, 2);

    // Move to South Fields (loc 5)
    sim.push_action(PlayerAction::Move { to: LocationId(7) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(9) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(3) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(5) });
    sim.step();
    sim.drain_results();

    // Diagnose South Fields (loc 5)
    sim.push_action(PlayerAction::Diagnose { location: LocationId(5) });
    sim.step();
    let diag_res = sim.drain_results();
    assert!(diag_res[0].success);

    // Draft HarvestDiagnosisReport
    let doc_type = DocumentType::HarvestDiagnosisReport {
        location: LocationId(5),
        finding: 2,
    };
    sim.push_action(PlayerAction::DraftDocument { doc_type: doc_type.clone() });
    sim.step();
    let draft_res = sim.drain_results();
    assert!(draft_res[0].success);

    // Verify document exists in DocumentRegistry
    {
        let docs = sim.world.resource::<DocumentRegistry>();
        assert_eq!(docs.documents.len(), 1);
        assert_eq!(docs.documents[0].doc_type, doc_type);
    }

    // Save to file, drop sim, reload
    let save_path = "saves/test_proof_b_save.gs2";
    fs::create_dir_all("saves").unwrap();
    sim.save(save_path).expect("Save must succeed");

    drop(sim);
    let mut loaded_sim = Simulation::load_from_file(save_path).expect("Load must succeed");

    // Verify DocumentRegistry survived save/load intact
    {
        let docs = loaded_sim.world.resource::<DocumentRegistry>();
        assert_eq!(docs.documents.len(), 1, "DocumentRegistry must have 1 document after reload");
        assert_eq!(docs.documents[0].doc_type, doc_type);
        assert!(docs.documents[0].signers.contains(&CitizenId::PLAYER));
    }

    // Register active CropBlightDispute consequence
    let consequence_id = {
        let current_tick = loaded_sim.tick();
        let mut reg = loaded_sim.world.resource_mut::<PendingConsequenceRegistry>();
        reg.register(
            201,
            TriggerCondition::TimeElapsed { duration_ticks: 72 },
            ConsequenceType::CropBlightDispute {
                farmer_a: CitizenId(3),
                farmer_b: CitizenId(13),
                location: LocationId(5),
            },
            current_tick,
        )
    };

    // Execute ArbitrateDispute
    loaded_sim.push_action(PlayerAction::ArbitrateDispute {
        document_id: 1,
        consequence_id,
    });
    loaded_sim.step();
    let arb_res = loaded_sim.drain_results();
    assert!(arb_res[0].success, "Arbitration must succeed: {:?}", arb_res);

    // Verify consequence resolved
    {
        let reg = loaded_sim.world.resource::<PendingConsequenceRegistry>();
        let c = reg.consequences.iter().find(|c| c.id == consequence_id).unwrap();
        assert_eq!(c.stage, ConsequenceStage::Resolved);
    }

    let _ = fs::remove_file(save_path);
}

// ── 4. Proof C Save/Load Continuity (Section 13) ────────────────────────────────

#[test]
fn test_proof_c_save_load_continuity() {
    let mut sim = Simulation::new();

    // Player aids Tomas with felling
    sim.push_action(PlayerAction::Move { to: LocationId(7) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(8) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(11) });
    sim.step();

    while sim.summary().hour < 8 {
        sim.push_action(PlayerAction::Wait { ticks: 1 });
        sim.step();
        sim.drain_results();
    }

    sim.push_action(PlayerAction::HelpWithFelling { npc: CitizenId(6) });
    sim.step();
    let res = sim.drain_results();
    assert!(res[0].success);

    // Return to inn
    sim.push_action(PlayerAction::Move { to: LocationId(9) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(1) });
    sim.step();
    sim.drain_results();

    // Advance 720 ticks (30 in-game days) with travel provisions
    for _ in 0..720 {
        sim.step();
        let mut q = sim.world.query_filtered::<(&mut PhysicalNeeds, &mut Demographics), bevy_ecs::prelude::With<PlayerMarker>>();
        for (mut needs, mut demo) in q.iter_mut(&mut sim.world) {
            needs.satiety = 100;
            demo.health = 100;
        }
    }

    // Save simulation after consequence maturation and return digest generation
    let save_path = "saves/test_proof_c_save.gs2";
    fs::create_dir_all("saves").unwrap();
    sim.save(save_path).expect("Save must succeed");

    drop(sim);
    let mut loaded_sim = Simulation::load_from_file(save_path).expect("Load must succeed");

    // Verify consequence is Matured
    {
        let reg = loaded_sim.world.resource::<PendingConsequenceRegistry>();
        assert_eq!(reg.consequences[0].stage, ConsequenceStage::Matured);
    }

    // Verify ReturnDigestLog is preserved across save/load
    {
        let digests = loaded_sim.world.resource::<ReturnDigestLog>();
        assert!(digests.find_digest_for(CitizenId(1)).is_some(), "Mira digest must exist after load");
    }

    // Wait until hour 14
    while loaded_sim.summary().hour != 14 {
        loaded_sim.push_action(PlayerAction::Wait { ticks: 1 });
        loaded_sim.step();
        loaded_sim.drain_results();
    }

    // Greet Mira Ashbridge
    loaded_sim.push_action(PlayerAction::Talk {
        npc: CitizenId(1),
        topic: TalkTopic::Greeting,
    });
    loaded_sim.step();
    let mira_first = loaded_sim.drain_results();
    assert!(mira_first[0].success);
    assert!(
        mira_first[0].message.contains("You've been gone a spell"),
        "Mira must deliver return digest greeting: {}",
        mira_first[0].message
    );

    // Verify digest consumed
    {
        let digests = loaded_sim.world.resource::<ReturnDigestLog>();
        assert!(digests.find_digest_for(CitizenId(1)).is_none(), "Mira digest must be consumed");
    }

    // Second greeting is steady-state
    loaded_sim.push_action(PlayerAction::Talk {
        npc: CitizenId(1),
        topic: TalkTopic::Greeting,
    });
    loaded_sim.step();
    let mira_second = loaded_sim.drain_results();
    assert!(mira_second[0].success);
    assert!(
        mira_second[0].message.contains("holding up at the woodlot"),
        "Second greeting must be steady-state: {}",
        mira_second[0].message
    );

    let _ = fs::remove_file(save_path);
}

// ── 5. V1 -> V2 Upward Migration Verification (Section 15) ─────────────────────

#[test]
fn test_v1_to_v2_migration_verification() {
    let mut sim = Simulation::new();
    sim.advance(48);

    // Build a V2 snapshot and convert to V1 for fixture creation
    let snap_v2 = sim.build_snapshot();
    let mut citizens_v1 = Vec::with_capacity(snap_v2.citizens.len());
    for c in snap_v2.citizens {
        citizens_v1.push(CitizenSnapshotV1 {
            meta: c.meta,
            demographics: c.demographics,
            household_ref: c.household_ref,
            settlement_ref: c.settlement_ref,
            occupation: c.occupation,
            finances: c.finances,
            needs: c.needs,
            mobility: c.mobility,
            kinship: c.kinship,
            causal_audit: c.causal_audit,
            inventory: c.inventory,
            npc_memory: c.npc_memory,
            npc_schedule: c.npc_schedule,
            npc_goals: c.npc_goals,
            disposition: c.disposition,
            is_player: c.is_player,
            capabilities: c.capabilities,
            transformation: c.transformation,
            knowledge: c.knowledge,
        });
    }

    let snap_v1 = SimulationSnapshotV1 {
        version: FORMAT_VERSION_V1,
        clock: snap_v2.clock,
        world_map: snap_v2.world_map,
        settlements: snap_v2.settlements,
        households: snap_v2.households,
        relationships: snap_v2.relationships,
        reputation: snap_v2.reputation,
        events: snap_v2.events,
        next_citizen_id: snap_v2.next_citizen_id,
        citizens: citizens_v1,
        seed: snap_v2.seed,
    };

    // Serialize to V1 format with MAGIC_V1 and FORMAT_VERSION_V1
    let mut buffer = Vec::new();
    save_snapshot_v1(&snap_v1, &mut buffer).expect("save_snapshot_v1 must succeed");

    // Load buffer using load_snapshot
    let mut cursor = Cursor::new(buffer);
    let migrated_v2 = load_snapshot(&mut cursor).expect("load_snapshot must migrate V1 to V2 without error");

    assert_eq!(migrated_v2.version, FORMAT_VERSION_V2);
    assert_eq!(migrated_v2.citizens.len(), 16);

    // Verify player and NPCs received sensible defaults
    let player_c = migrated_v2.citizens.iter().find(|c| c.is_player).unwrap();
    assert!(player_c.episodic_memory.is_none());
    assert!(player_c.relational_ledger.is_some());
    assert!(player_c.epistemic_state.is_some());

    let npc_c = migrated_v2.citizens.iter().find(|c| !c.is_player).unwrap();
    assert!(npc_c.episodic_memory.is_some());
    assert!(npc_c.relational_ledger.is_some());
    assert!(npc_c.epistemic_state.is_some());

    // Load into Simulation and verify invariants pass
    let mut loaded_sim = Simulation::from_snapshot(migrated_v2);
    assert!(loaded_sim.check_invariants().is_ok(), "Migrated V1 state must satisfy all invariants");

    // Advance 24 ticks on loaded simulation without panic
    loaded_sim.advance(24);
    assert!(loaded_sim.check_invariants().is_ok());
}

// ── 6. Malformed and Future Header Rejections (Section 16) ──────────────────────

#[test]
fn test_malformed_and_future_header_rejections() {
    // 1. Unknown magic header
    {
        let mut bytes = vec![0u8; 64];
        bytes[0..8].copy_from_slice(b"BADMAGIC");
        let mut cursor = Cursor::new(bytes);
        let err = load_snapshot(&mut cursor).unwrap_err();
        assert!(
            err.to_string().contains("Invalid save file magic"),
            "Error must specify invalid magic, got: {}",
            err
        );
    }

    // 2. Unsupported future version (e.g. version 3 with GODSEED2 magic)
    {
        let mut bytes = vec![0u8; 64];
        bytes[0..8].copy_from_slice(MAGIC_V2);
        bytes[8..12].copy_from_slice(&3u32.to_le_bytes());
        let mut cursor = Cursor::new(bytes);
        let err = load_snapshot(&mut cursor).unwrap_err();
        assert!(
            err.to_string().contains("Unsupported future save format version"),
            "Error must specify future version rejection, got: {}",
            err
        );
    }

    // 3. Truncated header (< 12 bytes)
    {
        let bytes = vec![0u8; 6];
        let mut cursor = Cursor::new(bytes);
        let err = load_snapshot(&mut cursor).unwrap_err();
        assert!(
            err.to_string().contains("Save file truncated or too short"),
            "Error must specify truncation, got: {}",
            err
        );
    }

    // 4. Bit-flipped CRC
    {
        let mut sim = Simulation::new();
        let snap = sim.build_snapshot();
        let mut buffer = Vec::new();
        save_snapshot(&snap, &mut buffer).unwrap();

        // Flip last byte of CRC
        let last_idx = buffer.len() - 1;
        buffer[last_idx] ^= 0xFF;

        let mut cursor = Cursor::new(buffer);
        let err = load_snapshot(&mut cursor).unwrap_err();
        assert!(
            err.to_string().contains("CRC32 mismatch"),
            "Error must specify CRC32 mismatch, got: {}",
            err
        );
    }

    // 5. Bit-flipped payload
    {
        let mut sim = Simulation::new();
        let snap = sim.build_snapshot();
        let mut buffer = Vec::new();
        save_snapshot(&snap, &mut buffer).unwrap();

        // Flip a byte in the middle of payload
        let mid_idx = buffer.len() / 2;
        buffer[mid_idx] ^= 0xAA;

        let mut cursor = Cursor::new(buffer);
        let err = load_snapshot(&mut cursor).unwrap_err();
        assert!(
            err.to_string().contains("CRC32 mismatch"),
            "Corrupted payload must fail CRC32 check, got: {}",
            err
        );
    }
}

// ── 7. Ghost Player Actions Fail Closed (Section 17 & 18) ────────────────────────

#[test]
fn test_ghost_player_actions_fail_closed() {
    let mut sim = Simulation::new();

    // Mark player as deceased
    {
        let mut q = sim.world.query_filtered::<&mut CitizenMeta, bevy_ecs::prelude::With<PlayerMarker>>();
        for mut meta in q.iter_mut(&mut sim.world) {
            meta.alive = false;
        }
    }

    let expected_msg = "You are deceased. The dead cannot act in the realm of the living.";

    // 1. Move
    sim.push_action(PlayerAction::Move { to: LocationId(3) });
    sim.step();
    let res = sim.drain_results();
    assert!(!res[0].success);
    assert_eq!(res[0].message, expected_msg);

    // 2. Talk
    sim.push_action(PlayerAction::Talk {
        npc: CitizenId(1),
        topic: TalkTopic::Greeting,
    });
    sim.step();
    let res = sim.drain_results();
    assert!(!res[0].success);
    assert_eq!(res[0].message, expected_msg);

    // 3. Buy
    sim.push_action(PlayerAction::Buy {
        resource: ResourceType::Food,
        quantity: 1,
    });
    sim.step();
    let res = sim.drain_results();
    assert!(!res[0].success);
    assert_eq!(res[0].message, expected_msg);

    // 4. Inscribe
    sim.push_action(PlayerAction::Inscribe {
        observation: "Test ghost inscription".to_string(),
    });
    sim.step();
    let res = sim.drain_results();
    assert!(!res[0].success);
    assert_eq!(res[0].message, expected_msg);

    // 5. HelpWithFelling
    sim.push_action(PlayerAction::HelpWithFelling { npc: CitizenId(6) });
    sim.step();
    let res = sim.drain_results();
    assert!(!res[0].success);
    assert_eq!(res[0].message, expected_msg);

    // 6. ArbitrateDispute
    sim.push_action(PlayerAction::ArbitrateDispute {
        document_id: 1,
        consequence_id: 1,
    });
    sim.step();
    let res = sim.drain_results();
    assert!(!res[0].success);
    assert_eq!(res[0].message, expected_msg);

    // 7. Practice
    sim.push_action(PlayerAction::Practice { capability: caps::INSCRIPTION });
    sim.step();
    let res = sim.drain_results();
    assert!(!res[0].success);
    assert_eq!(res[0].message, expected_msg);

    // 8. StudyArchive
    sim.push_action(PlayerAction::StudyArchive);
    sim.step();
    let res = sim.drain_results();
    assert!(!res[0].success);
    assert_eq!(res[0].message, expected_msg);
}

// ── 8. Consequence Guard & Arbitration Mismatch (Section 19) ────────────────────

#[test]
fn test_consequence_guard_and_arbitration_mismatch() {
    let mut sim = Simulation::new();

    // Move to Forest Edge (loc 11)
    sim.push_action(PlayerAction::Move { to: LocationId(7) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(8) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(11) });
    sim.step();
    sim.drain_results();

    while sim.summary().hour < 8 {
        sim.push_action(PlayerAction::Wait { ticks: 1 });
        sim.step();
        sim.drain_results();
    }

    // First HelpWithFelling: succeeds
    sim.push_action(PlayerAction::HelpWithFelling { npc: CitizenId(6) });
    sim.step();
    let res1 = sim.drain_results();
    assert!(res1[0].success);

    // Duplicate HelpWithFelling: fails closed without adding duplicate consequence
    sim.push_action(PlayerAction::HelpWithFelling { npc: CitizenId(6) });
    sim.step();
    let res2 = sim.drain_results();
    assert!(!res2[0].success);
    assert!(res2[0].message.contains("already assisted") || res2[0].message.contains("already unfolding"));

    // Verify exactly one consequence registered
    {
        let reg = sim.world.resource::<PendingConsequenceRegistry>();
        assert_eq!(reg.consequences.len(), 1);
    }

    // Arbitration Document Mismatch Test:
    // Create an arbitration situation with CropBlightDispute
    let consequence_id = {
        let current_tick = sim.tick();
        let mut reg = sim.world.resource_mut::<PendingConsequenceRegistry>();
        reg.register(
            201,
            TriggerCondition::TimeElapsed { duration_ticks: 72 },
            ConsequenceType::CropBlightDispute {
                farmer_a: CitizenId(3),
                farmer_b: CitizenId(13),
                location: LocationId(5),
            },
            current_tick,
        )
    };

    // Add a document of mismatched type (DebtReliefCharter) to DocumentRegistry
    let mismatched_doc_id = {
        let current_tick = sim.tick();
        let mut docs = sim.world.resource_mut::<DocumentRegistry>();
        docs.register(InscribedDocument {
            id: 0,
            doc_type: DocumentType::DebtReliefCharter {
                creditor: CitizenId(5),
                debtor: CitizenId(3),
                terms: 50,
            },
            drafter: CitizenId::PLAYER,
            signers: vec![CitizenId::PLAYER],
            binding_tick: current_tick,
            related_consequence_id: None,
        })
    };

    // Attempt arbitration with mismatched document type -> must fail closed
    sim.push_action(PlayerAction::ArbitrateDispute {
        document_id: mismatched_doc_id,
        consequence_id,
    });
    sim.step();
    let mismatch_res = sim.drain_results();
    assert!(!mismatch_res[0].success);
    assert!(
        mismatch_res[0].message.contains("not legally applicable")
            || mismatch_res[0].message.contains("does not address this dispute"),
        "Arbitration with mismatched document must fail closed: {}",
        mismatch_res[0].message
    );
}

// ── 9. Deterministic Save/Load Comparison (Section 21) ──────────────────────────

#[test]
fn test_deterministic_save_load_comparison_paths() {
    // Path A: Continuous run for 120 ticks
    let mut sim_a = Simulation::new();
    sim_a.advance(120);
    let hash_a = sim_a.state_hash();

    // Path B: Run 48 ticks -> Save -> Drop -> Load -> Run 72 ticks (total 120)
    let mut sim_b = Simulation::new();
    sim_b.advance(48);

    let save_path = "saves/test_determinism_path_b.gs2";
    fs::create_dir_all("saves").unwrap();
    sim_b.save(save_path).expect("Save must succeed");
    drop(sim_b);

    let mut sim_b_loaded = Simulation::load_from_file(save_path).expect("Load must succeed");
    sim_b_loaded.advance(72);
    let hash_b = sim_b_loaded.state_hash();

    assert_eq!(
        hash_a, hash_b,
        "Bit-for-bit determinism must hold identically between continuous Path A and interrupted Path B"
    );

    let _ = fs::remove_file(save_path);
}
