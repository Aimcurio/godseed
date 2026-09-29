use godseed_core::{
    components::{
        CapabilitySet, CitizenMeta, EpisodicMemory, EpistemicState,
        PlayerMarker, RelationalLedger, TransformationState,
    },
    content::{caps, milestones},
    invariants::verify_invariants,
    resources::{DocumentRegistry, PendingConsequenceRegistry},
    sim::Simulation,
    types::{
        CapabilityLevel, CitizenId, ConsequenceStage, ConsequenceType,
        DocumentType, LocationId, MemoryTag, PlayerAction, TriggerCondition,
    },
};

#[test]
fn test_scholar_stage2_diagnosis_and_dispute_arbitration() {
    let mut sim = Simulation::new();

    // ── Phase 1: Advance to Scholar Stage 2 (The Settlement Chronicler) ───────

    // Step 1: Study the archive to discover inscription path
    sim.push_action(PlayerAction::Move { to: LocationId(7) }); // Well
    sim.step();
    sim.drain_results();

    sim.push_action(PlayerAction::Move { to: LocationId(8) }); // Old Archive
    sim.step();
    sim.drain_results();

    sim.push_action(PlayerAction::StudyArchive);
    sim.step();
    let archive_res = sim.drain_results();
    assert!(archive_res[0].success, "Studying archive should succeed");

    // Step 2: Practice and acquire Inscription capability
    sim.push_action(PlayerAction::Practice { capability: caps::INSCRIPTION });
    sim.step();
    sim.drain_results();

    // Advance 30 ticks to trigger monthly transformation check -> Stage 1 (Scholar)
    sim.advance(30);
    {
        let summary = sim.summary();
        let player = summary.player.unwrap();
        assert_eq!(player.transformation_stage, 1, "Player must advance to Scholar Stage 1");
    }

    // Step 3: Complete 5 inscriptions
    for i in 1..=5 {
        sim.push_action(PlayerAction::Inscribe {
            observation: format!("Field observation #{}: Inscribing communal dynamics in Thornveil.", i),
        });
        sim.step();
        let inscribe_res = sim.drain_results();
        assert!(inscribe_res[0].success);
    }

    // Step 4: Build relationship with Elder Voss (CitizenId 5) > 60
    // Elder Voss is at Location 3 (Market) or Location 1 (Inn) depending on hour
    {
        use godseed_core::resources::RelationshipLedger;
        let mut rels = sim.world.resource_mut::<RelationshipLedger>();
        rels.set(CitizenId::PLAYER, CitizenId(5), 75);
    }

    // Advance 30 ticks to trigger monthly check -> Stage 2 (The Settlement Chronicler)
    sim.advance(30);
    {
        let summary = sim.summary();
        let player = summary.player.unwrap();
        assert_eq!(
            player.transformation_stage, 2,
            "Player must advance to Scholar Stage 2 (The Settlement Chronicler) upon 5 inscriptions + Elder recognition"
        );

        let mut q = sim.world.query_filtered::<(&CapabilitySet, &TransformationState), bevy_ecs::query::With<PlayerMarker>>();
        let (caps_set, transform) = q.iter(&sim.world).next().unwrap();
        assert!(caps_set.has(caps::DIAGNOSIS), "Stage 2 Chronicler must possess DIAGNOSIS capability");
        assert!(
            caps_set.level(caps::INSCRIPTION) >= CapabilityLevel::JOURNEYMAN,
            "Chronicler must have Journeyman-level Inscription"
        );
        assert!(
            transform.milestones.contains(&milestones::SCHOLAR_RECOGNIZED),
            "Must possess SCHOLAR_RECOGNIZED milestone"
        );
    }

    // ── Phase 2: On-site Diagnosis at South Fields (Location 5) ──────────────

    // Move to South Fields: Archive(8) -> Well(7) -> Road(9) -> Market(3) -> South Fields(5)
    sim.push_action(PlayerAction::Move { to: LocationId(7) });
    sim.step();
    sim.drain_results();

    sim.push_action(PlayerAction::Move { to: LocationId(9) });
    sim.step();
    sim.drain_results();

    sim.push_action(PlayerAction::Move { to: LocationId(3) });
    sim.step();
    sim.drain_results();

    sim.push_action(PlayerAction::Move { to: LocationId(5) });
    sim.step();
    sim.drain_results();

    // Perform diagnostic examination
    sim.push_action(PlayerAction::Diagnose { location: LocationId(5) });
    sim.step();
    let diag_res = sim.drain_results();
    assert!(diag_res[0].success, "Diagnosing South Fields must succeed");
    assert!(
        diag_res[0].message.contains("fungal blight vulnerability") || diag_res[0].message.contains("Agricultural Diagnosis"),
        "Diagnostic message must articulate systemic agrarian condition"
    );

    // Verify player learned knowledge node 2 (Crop Blight Vulnerability)
    {
        let mut q = sim.world.query_filtered::<&EpistemicState, bevy_ecs::query::With<PlayerMarker>>();
        let epistemic = q.iter(&sim.world).next().unwrap();
        assert!(
            epistemic.has_knowledge(2),
            "Diagnosing must register knowledge node 2 in EpistemicState"
        );
    }

    // ── Phase 3: Drafting Legally Binding Inscribed Document ──────────────────

    let doc_type = DocumentType::HarvestDiagnosisReport {
        location: LocationId(5),
        finding: 2,
    };

    sim.push_action(PlayerAction::DraftDocument { doc_type: doc_type.clone() });
    sim.step();
    let draft_res = sim.drain_results();
    assert!(draft_res[0].success, "Drafting harvest diagnosis report must succeed: {}", draft_res[0].message);
    assert!(draft_res[0].message.contains("Inscribed Document #1"), "Got: {}", draft_res[0].message);

    // Verify DocumentRegistry contains document #1
    {
        let doc_reg = sim.world.resource::<DocumentRegistry>();
        let doc = doc_reg.get(1).expect("Document #1 must exist in DocumentRegistry");
        assert_eq!(doc.id, 1);
        assert_eq!(doc.drafter, CitizenId::PLAYER);
        assert!(doc.signers.contains(&CitizenId::PLAYER));
        assert_eq!(doc.doc_type, doc_type);
        assert_eq!(doc.related_consequence_id, None);
    }

    // ── Phase 4: Autonomous Crop Blight Dispute & Legal Arbitration (AC-205) ─

    // Register an active dispute consequence between Farmer Oswin Cley (Citizen 3)
    // and Fieldworker Corva (Citizen 13) regarding blighted lower furrows
    let dispute_id = {
        let current_tick = sim.world.resource::<godseed_core::types::SimClock>().tick;
        let mut cons_reg = sim.world.resource_mut::<PendingConsequenceRegistry>();
        cons_reg.register(
            201, // causal root
            TriggerCondition::TimeElapsed { duration_ticks: 72 },
            ConsequenceType::CropBlightDispute {
                farmer_a: CitizenId(3),
                farmer_b: CitizenId(13),
                location: LocationId(5),
            },
            current_tick,
        )
    };

    // Verify dispute is Active before arbitration
    {
        let cons_reg = sim.world.resource::<PendingConsequenceRegistry>();
        let dispute = cons_reg.consequences.iter().find(|c| c.id == dispute_id).unwrap();
        assert_eq!(dispute.stage, ConsequenceStage::Active);
    }

    // Present document #1 to arbitrate dispute #dispute_id
    sim.push_action(PlayerAction::ArbitrateDispute {
        document_id: 1,
        consequence_id: dispute_id,
    });
    sim.step();
    let arbitrate_res = sim.drain_results();
    assert!(arbitrate_res[0].success, "Arbitrating dispute must succeed");
    assert!(
        arbitrate_res[0].message.contains("RESOLVED"),
        "Arbitration message must confirm formal resolution"
    );

    // ── Phase 5: Verification of Documentary Authority & Social Transformation ─

    // 1. Consequence stage must be Resolved
    {
        let cons_reg = sim.world.resource::<PendingConsequenceRegistry>();
        let dispute = cons_reg.consequences.iter().find(|c| c.id == dispute_id).unwrap();
        assert_eq!(
            dispute.stage,
            ConsequenceStage::Resolved,
            "Dispute must be marked ConsequenceStage::Resolved"
        );
    }

    // 2. Document must be marked with related consequence and signed by both parties
    {
        let doc_reg = sim.world.resource::<DocumentRegistry>();
        let doc = doc_reg.get(1).unwrap();
        assert_eq!(doc.related_consequence_id, Some(dispute_id));
        assert!(doc.signers.contains(&CitizenId::PLAYER));
        assert!(doc.signers.contains(&CitizenId(3)), "Citizen 3 must have affixed mark");
        assert!(doc.signers.contains(&CitizenId(13)), "Citizen 13 must have affixed mark");
    }

    // 3. Both citizens must hold permanent turning point episodic memory
    {
        let mut q = sim.world.query::<(&CitizenMeta, &EpisodicMemory, &RelationalLedger)>();
        let mut found_3 = false;
        let mut found_13 = false;

        for (meta, mem, ledger) in q.iter(&sim.world) {
            if meta.id == CitizenId(3) || meta.id == CitizenId(13) {
                if meta.id == CitizenId(3) { found_3 = true; }
                if meta.id == CitizenId(13) { found_13 = true; }

                // Must have permanent ContractSigned anchor memory
                let has_contract_anchor = mem.anchors.iter().any(|a| {
                    a.tag == MemoryTag::ContractSigned && a.narrative_token == 205 && a.is_permanent
                });
                assert!(
                    has_contract_anchor,
                    "Citizen {} must possess permanent ContractSigned anchor memory from arbitration",
                    meta.id.0
                );

                // Triad RelationalBond with player (Citizen 0) must reflect elevated trust and sentiment
                let bond = ledger.bonds.get(&0).expect("Bond with player must exist");
                assert!(
                    bond.trust >= 30,
                    "Citizen {} trust toward player must be elevated (got {})",
                    meta.id.0, bond.trust
                );
                assert!(
                    bond.sentiment >= 20,
                    "Citizen {} sentiment toward player must be positive (got {})",
                    meta.id.0, bond.sentiment
                );
            }
        }
        assert!(found_3 && found_13, "Both citizens 3 and 13 must have been evaluated");
    }

    // ── Phase 6: Long-term Stability (720 ticks / 30 simulated days) ─────────

    sim.advance(720);

    // Consequence must remain Resolved (not re-escalated or broken)
    {
        let cons_reg = sim.world.resource::<PendingConsequenceRegistry>();
        let dispute = cons_reg.consequences.iter().find(|c| c.id == dispute_id).unwrap();
        assert_eq!(dispute.stage, ConsequenceStage::Resolved);
    }

    // All machine-checkable invariants INV-1 through INV-7 must strictly hold
    let violations = verify_invariants(&mut sim.world);
    assert!(
        violations.is_ok(),
        "Simulation must maintain 100% invariant compliance across arbitration lifecycle: {:?}",
        violations
    );
}
