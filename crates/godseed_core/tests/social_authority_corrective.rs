use godseed_core::{
    components::{
        CitizenMeta, Disposition, EpisodicMemory, EpistemicState, NpcMemory, NpcSchedule,
        NpcSocialProfile, OccupationProfile, RelationalLedger,
    },
    events::SimEvent,
    persistence::{
        load_snapshot, save_snapshot_v1, CitizenSnapshotV1, SimulationSnapshotV1,
        FORMAT_VERSION_V1, FORMAT_VERSION_V3,
    },
    resources::{EventRing, PendingConsequenceRegistry, RelationshipLedger},
    sim::Simulation,
    types::{
        BehavioralMode, CitizenId, ConsequenceStage, KnowledgeNodeId, LocationId, MemoryTag,
        NpcActivity, OccupationType, PlayerAction, RelationalBond, ScheduleSlot, TalkTopic,
    },
};
use std::io::Cursor;

// ── 1. AC-201 Qualitative Relational Divergence ──────────────────────────────

#[test]
fn test_ac201_relational_divergence() {
    let mut sim = Simulation::new();

    // Move player to Inn (LocationId 1)
    sim.push_action(PlayerAction::Move { to: LocationId(1) });
    sim.step();
    sim.drain_results();

    let npc_a = CitizenId(1); // Mira Ashbridge
    let npc_b = CitizenId(4); // Sera Cley

    {
        let mut query = sim.world.query::<(
            &CitizenMeta,
            &mut godseed_core::components::SettlementRef,
            &mut NpcSchedule,
            &mut RelationalLedger,
        )>();
        for (meta, mut sref, mut sched, mut ledger) in query.iter_mut(&mut sim.world) {
            if meta.id == npc_a {
                sref.current_location = LocationId(1);
                sched.home_location = LocationId(1);
                sched.work_location = LocationId(1);
                sched.slots = vec![ScheduleSlot {
                    tick_start: 0,
                    activity: NpcActivity::Socializing,
                    location: LocationId(1),
                }];
                ledger.set_bond(
                    CitizenId::PLAYER,
                    RelationalBond {
                        sentiment: 50,
                        trust: -30,
                        obligation: 0,
                    },
                );
            } else if meta.id == npc_b {
                sref.current_location = LocationId(1);
                sched.home_location = LocationId(1);
                sched.work_location = LocationId(1);
                sched.slots = vec![ScheduleSlot {
                    tick_start: 0,
                    activity: NpcActivity::Socializing,
                    location: LocationId(1),
                }];
                ledger.set_bond(
                    CitizenId::PLAYER,
                    RelationalBond {
                        sentiment: -30,
                        trust: 0,
                        obligation: 60,
                    },
                );
            }
        }
    }

    // Verify behavioural modes under VS2 multi-dimensional model
    {
        let mut query = sim.world.query::<(&CitizenMeta, &RelationalLedger)>();
        for (meta, ledger) in query.iter(&sim.world) {
            if meta.id == npc_a {
                let bond = ledger.get_bond(CitizenId::PLAYER);
                assert_eq!(
                    bond.mode(),
                    BehavioralMode::AffectionateRefusal,
                    "NPC A must be in AffectionateRefusal mode"
                );
            } else if meta.id == npc_b {
                let bond = ledger.get_bond(CitizenId::PLAYER);
                assert_eq!(
                    bond.mode(),
                    BehavioralMode::GrudgingDebtor,
                    "NPC B must be in GrudgingDebtor mode"
                );
            }
        }
    }

    // Scalar formula calculation: (sentiment + trust) / 2
    // Under scalar model:
    // NPC A score = (50 + -30) / 2 = +10 (would accept work)
    // NPC B score = (-30 + 0) / 2 = -15 (would reject work)
    //
    // Under Godseed VS2 qualitative authority, these behaviors strictly invert:
    // NPC A (fond but distrusting) warmly refuses RequestWork.
    // NPC B (hostile but indebted) grudgingly complies with RequestWork and reduces debt.

    // Test NPC A (Alden Croft) RequestWork -> Warm Refusal
    sim.push_action(PlayerAction::Talk {
        npc: npc_a,
        topic: TalkTopic::RequestWork,
    });
    sim.step();
    let res_a = sim.drain_results();
    assert_eq!(res_a.len(), 1);
    assert!(
        res_a[0]
            .message
            .contains("fond of you, truly. But I cannot trust you with this work"),
        "NPC A must provide AffectionateRefusal dialog, got: {}",
        res_a[0].message
    );

    // Test NPC B (Orin Frost) RequestWork -> Begrudging Compliance
    sim.push_action(PlayerAction::Talk {
        npc: npc_b,
        topic: TalkTopic::RequestWork,
    });
    sim.step();
    let res_b = sim.drain_results();
    assert_eq!(res_b.len(), 1);
    assert!(
        res_b[0]
            .message
            .contains("I don't like you, but I honor my debts. Fine, take the work."),
        "NPC B must comply begrudgingly due to debt obligation, got: {}",
        res_b[0].message
    );

    // Verify obligation deduction on GrudgingDebtor compliance (deducts 20 obligation)
    {
        let mut query = sim.world.query::<(&CitizenMeta, &RelationalLedger)>();
        for (meta, ledger) in query.iter(&sim.world) {
            if meta.id == npc_b {
                let bond = ledger.get_bond(CitizenId::PLAYER);
                assert_eq!(
                    bond.obligation, 40,
                    "Providing work must reduce obligation from 60 to 40"
                );
            }
        }
    }
}

// ── 2. AC-202 Episodic Recall Changes Behavior After >= 14 Days ───────────────

#[test]
fn test_ac202_episodic_recall_changes_behavior() {
    let mut sim = Simulation::new();

    // Move to Forest Edge (11)
    sim.push_action(PlayerAction::Move { to: LocationId(7) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(8) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(11) });
    sim.step();
    sim.drain_results();

    // Advance to hour 8 when Tomas Birch works at Forest Edge
    while sim.summary().hour < 8 {
        sim.push_action(PlayerAction::Wait { ticks: 1 });
        sim.step();
        sim.drain_results();
    }

    // Perform felling assistance
    sim.push_action(PlayerAction::HelpWithFelling { npc: CitizenId(6) });
    sim.step();
    assert!(sim.drain_results()[0].success);

    // Advance >= 14 simulated days (336 ticks)
    sim.advance(336);

    // Set bond to WaryConsultant with 0 trust to prove episodic memory is decisive
    {
        let mut query = sim.world.query::<(&CitizenMeta, &mut RelationalLedger)>();
        for (meta, mut ledger) in query.iter_mut(&mut sim.world) {
            if meta.id == CitizenId(6) {
                ledger.set_bond(
                    CitizenId::PLAYER,
                    RelationalBond {
                        sentiment: 0,
                        trust: -5,
                        obligation: 0,
                    },
                );
            }
        }
    }

    // Verify Tomas Birch cites the felling and accepts work because of episodic memory
    sim.push_action(PlayerAction::Talk {
        npc: CitizenId(6),
        topic: TalkTopic::RequestWork,
    });
    sim.step();
    let res = sim.drain_results();
    assert!(
        res[0]
            .message
            .contains("After what you did with the great oak, my work is always open to you."),
        "Tomas must cite felling assistance and accept work, got: {}",
        res[0].message
    );
}

// ── 3. Permanent Turning Point Survives Anchor Pressure (Option A) ────────────

#[test]
fn test_permanent_turning_point_survives_anchor_pressure() {
    let mut mem = EpisodicMemory::new();

    // 1. Add permanent turning point with high impact
    let felling_record = godseed_core::types::EpisodicRecord {
        id: 1,
        tick: 10,
        actor: CitizenId(6),
        target: Some(CitizenId::PLAYER),
        tag: MemoryTag::HelpedWithFelling,
        delta_sentiment: 50,
        delta_trust: 50,
        delta_obligation: 20,
        is_permanent: true,
        narrative_token: 101,
        causal: None,
    };
    mem.add_record(felling_record);

    // 2. Flood with 10 additional permanent anchors with smaller impact
    for i in 2..=12 {
        let rec = godseed_core::types::EpisodicRecord {
            id: i,
            tick: 10 + i,
            actor: CitizenId(6),
            target: Some(CitizenId::PLAYER),
            tag: MemoryTag::ContractSigned,
            delta_sentiment: 5,
            delta_trust: 5,
            delta_obligation: 0,
            is_permanent: true,
            narrative_token: 200,
            causal: None,
        };
        mem.add_record(rec);
    }

    // 3. Flood with 20 transient records
    for i in 20..40 {
        let rec = godseed_core::types::EpisodicRecord {
            id: i,
            tick: 100 + i,
            actor: CitizenId(6),
            target: Some(CitizenId::PLAYER),
            tag: MemoryTag::CasualInteraction,
            delta_sentiment: 1,
            delta_trust: 1,
            delta_obligation: 0,
            is_permanent: false,
            narrative_token: 300,
            causal: None,
        };
        mem.add_record(rec);
    }

    // 4. Verify permanent turning point survived anchor pressure under Option A
    assert!(
        mem.has_anchor_with_tag(MemoryTag::HelpedWithFelling),
        "HelpedWithFelling anchor must survive indefinite anchor pressure"
    );
    assert!(
        mem.has_record_with_tag(MemoryTag::HelpedWithFelling),
        "HelpedWithFelling record must remain discoverable"
    );
    assert_eq!(
        mem.anchors.len(),
        6,
        "Active anchors must remain bounded at 6"
    );
    assert_eq!(
        mem.transient.len(),
        12,
        "Transient FIFO must remain bounded at 12"
    );
    assert!(
        !mem.compacted_anchors.is_empty(),
        "Compacted permanent anchors must contain evicted overflow"
    );
}

// ── 4. AC-203 One-Hop Narrative Gossip ────────────────────────────────────────

#[test]
fn test_ac203_one_hop_narrative_gossip() {
    let mut sim = Simulation::new();

    // 1. Move to Forest Edge (11) and assist Tomas Birch (CitizenId 6)
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

    sim.push_action(PlayerAction::HelpWithFelling { npc: CitizenId(6) });
    sim.step();
    assert!(sim.drain_results()[0].success);

    // 2. Teleport / place Tomas Birch and Elin Finch (CitizenId 5) together at Tavern (LocationId 1)
    // while set to Socializing activity
    {
        let mut query = sim.world.query::<(
            &CitizenMeta,
            &mut godseed_core::components::SettlementRef,
            &mut NpcSchedule,
        )>();
        for (meta, mut sref, mut sched) in query.iter_mut(&mut sim.world) {
            if meta.id == CitizenId(6) || meta.id == CitizenId(5) {
                sref.current_location = LocationId(1);
                sched.home_location = LocationId(1);
                sched.work_location = LocationId(1);
                sched.slots = vec![ScheduleSlot {
                    tick_start: 0,
                    activity: NpcActivity::Socializing,
                    location: LocationId(1),
                }];
            }
        }
    }

    // 3. Run gossip schedule
    sim.step_weekly();

    // 4. Verify Elin Finch received the firsthand gossip record from Tomas
    let elin_has_gossip = {
        let mut query = sim.world.query::<(&CitizenMeta, &EpisodicMemory)>();
        let mut found = false;
        for (meta, mem) in query.iter(&sim.world) {
            if meta.id == CitizenId(5) {
                found = mem.has_record_with_tag(MemoryTag::HeardGossipAbout(CitizenId(6)));
            }
        }
        found
    };
    assert!(
        elin_has_gossip,
        "Elin Finch must receive narrative gossip about Tomas Birch"
    );

    // 5. Verify Elin Finch's first greeting cites the gossip before any direct interaction
    // Move player to Tavern (LocationId 1)
    sim.push_action(PlayerAction::Move { to: LocationId(8) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(7) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(9) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(1) });
    sim.step();
    sim.drain_results();

    sim.push_action(PlayerAction::Talk {
        npc: CitizenId(5),
        topic: TalkTopic::Greeting,
    });
    sim.step();
    let greeting_res = sim.drain_results();
    assert!(
        greeting_res[0]
            .message
            .contains("Tomas told me what you did at the woodlot"),
        "Elin must cite Tomas's story in greeting: {}",
        greeting_res[0].message
    );

    // 6. Enforce one-hop bound: another NPC (e.g. Delia Croft, CitizenId 7) at a different location
    // must NOT receive the gossip transitively
    let delia_has_gossip = {
        let mut query = sim.world.query::<(&CitizenMeta, &EpisodicMemory)>();
        let mut found = false;
        for (meta, mem) in query.iter(&sim.world) {
            if meta.id == CitizenId(7) {
                found = mem.has_record_with_tag(MemoryTag::HeardGossipAbout(CitizenId(6)))
                    || mem.has_record_with_tag(MemoryTag::HeardGossipAbout(CitizenId(5)));
            }
        }
        found
    };
    assert!(
        !delia_has_gossip,
        "Delia Croft must not receive transitive gossip in the same tick"
    );
}

// ── 5. Fail-Closed Unknown Knowledge Sharing ──────────────────────────────────

#[test]
fn test_share_unknown_knowledge_fails_closed() {
    let mut sim = Simulation::new();

    // Move player to Inn (LocationId 1) where Mira is
    sim.push_action(PlayerAction::Move { to: LocationId(1) });
    sim.step();
    sim.drain_results();

    // Player does not possess KnowledgeNodeId(99)
    sim.push_action(PlayerAction::Talk {
        npc: CitizenId(1),
        topic: TalkTopic::ShareKnowledge {
            node: KnowledgeNodeId(99),
        },
    });
    sim.step();
    let res = sim.drain_results();
    assert_eq!(res.len(), 1);
    assert!(
        !res[0].success,
        "Sharing unknown knowledge must fail closed"
    );
    assert_eq!(
        res[0].message,
        "You don't know enough about that to share it."
    );

    // Verify no KnowledgeShared event was emitted
    let ring = sim.world.resource::<EventRing>();
    assert!(
        !ring.events.iter().any(|e| matches!(
            e,
            SimEvent::KnowledgeShared {
                knowledge_id: 99,
                ..
            }
        )),
        "No event must be emitted for rejected share action"
    );
}

// ── 6. AC-204 Asymmetric Debt Leverage on Delia Croft ─────────────────────────

#[test]
fn test_ac204_asymmetric_knowledge_leverage() {
    let mut sim = Simulation::new();

    // 1. Initially, player talks to Delia Croft (CitizenId 7) at Market Square (LocationId 3)
    // Move directly from Road (9) to Market Square (3)
    sim.push_action(PlayerAction::Move { to: LocationId(3) });
    sim.step();
    sim.drain_results();

    // Ensure Delia Croft is at Market Square (LocationId 3)
    {
        let mut query = sim.world.query::<(
            &CitizenMeta,
            &mut godseed_core::components::SettlementRef,
            &mut NpcSchedule,
        )>();
        for (meta, mut sref, mut sched) in query.iter_mut(&mut sim.world) {
            if meta.id == CitizenId(7) {
                sref.current_location = LocationId(3);
                for slot in &mut sched.slots {
                    slot.location = LocationId(3);
                }
            }
        }
    }

    // Delia without leverage refuses concession
    sim.push_action(PlayerAction::Talk {
        npc: CitizenId(7),
        topic: TalkTopic::RequestWork,
    });
    sim.step();
    let initial_res = sim.drain_results();
    assert!(
        !initial_res[0]
            .message
            .contains("market concession—just keep your silence about my debt"),
        "Delia must not concede before player leverages debt: {}",
        initial_res[0].message
    );

    // 2. Grant player Knowledge 6 (Elder Ledger Secrets)
    let tick = sim.tick();
    {
        let mut query = sim.world.query_filtered::<
            &mut godseed_core::components::EpistemicState,
            bevy_ecs::prelude::With<godseed_core::components::PlayerMarker>,
        >();
        for mut epistemic in query.iter_mut(&mut sim.world) {
            epistemic.learn(6, tick);
        }
    }

    // 3. Share Knowledge 6 with Delia Croft
    sim.push_action(PlayerAction::Talk {
        npc: CitizenId(7),
        topic: TalkTopic::ShareKnowledge {
            node: KnowledgeNodeId(6),
        },
    });
    sim.step();
    let leverage_res = sim.drain_results();
    assert!(
        leverage_res[0].success,
        "Sharing debt knowledge with Delia must succeed"
    );
    assert!(
        leverage_res[0]
            .message
            .contains("Where did you hear about that debt...?"),
        "Delia must acknowledge debt leverage: {}",
        leverage_res[0].message
    );

    // 4. Verify Delia Croft's obligation shifted by 80 into GrudgingDebtor
    {
        let mut query = sim.world.query::<(&CitizenMeta, &RelationalLedger)>();
        for (meta, ledger) in query.iter(&sim.world) {
            if meta.id == CitizenId(7) {
                let bond = ledger.get_bond(CitizenId::PLAYER);
                assert!(
                    bond.obligation >= 80,
                    "Delia's obligation must increase by 80, got: {}",
                    bond.obligation
                );
                assert_eq!(
                    bond.mode(),
                    BehavioralMode::GrudgingDebtor,
                    "Delia must enter GrudgingDebtor mode"
                );
            }
        }
    }

    // 5. Delia now concedes on RequestWork
    sim.push_action(PlayerAction::Talk {
        npc: CitizenId(7),
        topic: TalkTopic::RequestWork,
    });
    sim.step();
    let concession_res = sim.drain_results();
    assert!(
        concession_res[0]
            .message
            .contains("keep your silence about my debt"),
        "Delia must grant work and market concession under debt leverage: {}",
        concession_res[0].message
    );
}

// ── 7. Single Social Authority in V2/V3 Runtime (ECS Clean) ───────────────────

#[test]
fn test_v2_runtime_has_single_social_authority() {
    let mut sim = Simulation::new();

    // 1. Verify fresh simulation has 0 Disposition and 0 NpcMemory components
    let disp_count = sim.world.query::<&Disposition>().iter(&sim.world).count();
    let npc_mem_count = sim.world.query::<&NpcMemory>().iter(&sim.world).count();
    assert_eq!(
        disp_count, 0,
        "Fresh simulation must have 0 Disposition components"
    );
    assert_eq!(
        npc_mem_count, 0,
        "Fresh simulation must have 0 NpcMemory components"
    );

    // Verify RelationshipLedger resource is absent
    assert!(
        sim.world.get_resource::<RelationshipLedger>().is_none(),
        "RelationshipLedger must not be installed in runtime world"
    );

    // Verify every NPC has NpcSocialProfile, EpisodicMemory, RelationalLedger, EpistemicState
    let mut npc_query = sim.world.query::<(
        &CitizenMeta,
        &NpcSocialProfile,
        &EpisodicMemory,
        &RelationalLedger,
        &EpistemicState,
    )>();
    let count = npc_query.iter(&sim.world).count();
    assert_eq!(
        count, 15,
        "All 15 NPCs must have authoritative VS2 components"
    );

    // 2. Save snapshot, load into new sim, verify persistence maintains zero legacy components
    let snapshot = sim.build_snapshot();
    assert_eq!(snapshot.version, FORMAT_VERSION_V3, "Snapshot must be V3");

    let mut loaded_sim = Simulation::from_snapshot(snapshot);
    let loaded_disp_count = loaded_sim
        .world
        .query::<&Disposition>()
        .iter(&loaded_sim.world)
        .count();
    let loaded_npc_mem_count = loaded_sim
        .world
        .query::<&NpcMemory>()
        .iter(&loaded_sim.world)
        .count();
    assert_eq!(
        loaded_disp_count, 0,
        "Loaded simulation must have 0 Disposition components"
    );
    assert_eq!(
        loaded_npc_mem_count, 0,
        "Loaded simulation must have 0 NpcMemory components"
    );
    assert!(
        loaded_sim
            .world
            .get_resource::<RelationshipLedger>()
            .is_none(),
        "Loaded simulation must have no RelationshipLedger resource"
    );
}

// ── 8. Contradictory Legacy Values Cannot Override Canonical V2 Authority ──────

#[test]
fn test_legacy_positive_cannot_override_vs2_enemy() {
    let mut sim = Simulation::new();

    // Take a V3 snapshot and convert to V1 with an artificial high positive legacy relationship
    let snap_v3 = sim.build_snapshot();
    let mut citizens_v1 = Vec::with_capacity(snap_v3.citizens.len());
    for c in snap_v3.citizens {
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
            npc_memory: None,
            npc_schedule: c.npc_schedule,
            npc_goals: c.npc_goals,
            disposition: None,
            is_player: c.is_player,
            capabilities: c.capabilities,
            transformation: c.transformation,
            knowledge: c.knowledge,
        });
    }

    let mut legacy_ledger = RelationshipLedger::default();
    legacy_ledger.set(CitizenId(1), CitizenId::PLAYER, 95); // High positive legacy score!

    let snap_v1 = SimulationSnapshotV1 {
        version: FORMAT_VERSION_V1,
        clock: snap_v3.clock,
        world_map: snap_v3.world_map,
        settlements: snap_v3.settlements,
        households: snap_v3.households,
        relationships: legacy_ledger,
        reputation: snap_v3.reputation,
        events: snap_v3.events,
        next_citizen_id: snap_v3.next_citizen_id,
        citizens: citizens_v1,
        seed: snap_v3.seed,
    };

    let mut buf = Vec::new();
    save_snapshot_v1(&snap_v1, &mut buf).unwrap();

    let mut cursor = Cursor::new(buf);
    let migrated_v3 = load_snapshot(&mut cursor).unwrap();
    let mut migrated_sim = Simulation::from_snapshot(migrated_v3);

    // Move player to Inn (LocationId 1) where Mira is
    migrated_sim.push_action(PlayerAction::Move { to: LocationId(1) });
    migrated_sim.step();
    migrated_sim.drain_results();

    // Set canonical bond on Mira Ashbridge (CitizenId 1) to HardenedEnemy
    {
        let mut query = migrated_sim
            .world
            .query::<(&CitizenMeta, &mut RelationalLedger)>();
        for (meta, mut ledger) in query.iter_mut(&mut migrated_sim.world) {
            if meta.id == CitizenId(1) {
                ledger.set_bond(
                    CitizenId::PLAYER,
                    RelationalBond {
                        sentiment: -80,
                        trust: -80,
                        obligation: 0,
                    },
                );
            }
        }
    }

    // Verify Alden behaves strictly as HardenedEnemy despite legacy score of +95
    migrated_sim.push_action(PlayerAction::Talk {
        npc: CitizenId(1),
        topic: TalkTopic::RequestWork,
    });
    migrated_sim.step();
    let res = migrated_sim.drain_results();
    assert!(
        res[0]
            .message
            .contains("Get away from me. I'd burn my tools before hiring you."),
        "HardenedEnemy must govern; legacy positive score must have 0 effect: {}",
        res[0].message
    );
}

#[test]
fn test_legacy_negative_cannot_override_vs2_ally() {
    let mut sim = Simulation::new();

    // Take a V3 snapshot and convert to V1 with an artificial low negative legacy relationship
    let snap_v3 = sim.build_snapshot();
    let mut citizens_v1 = Vec::with_capacity(snap_v3.citizens.len());
    for c in snap_v3.citizens {
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
            npc_memory: None,
            npc_schedule: c.npc_schedule,
            npc_goals: c.npc_goals,
            disposition: None,
            is_player: c.is_player,
            capabilities: c.capabilities,
            transformation: c.transformation,
            knowledge: c.knowledge,
        });
    }

    let mut legacy_ledger = RelationshipLedger::default();
    legacy_ledger.set(CitizenId(1), CitizenId::PLAYER, -95); // Extremely negative legacy score!

    let snap_v1 = SimulationSnapshotV1 {
        version: FORMAT_VERSION_V1,
        clock: snap_v3.clock,
        world_map: snap_v3.world_map,
        settlements: snap_v3.settlements,
        households: snap_v3.households,
        relationships: legacy_ledger,
        reputation: snap_v3.reputation,
        events: snap_v3.events,
        next_citizen_id: snap_v3.next_citizen_id,
        citizens: citizens_v1,
        seed: snap_v3.seed,
    };

    let mut buf = Vec::new();
    save_snapshot_v1(&snap_v1, &mut buf).unwrap();

    let mut cursor = Cursor::new(buf);
    let migrated_v3 = load_snapshot(&mut cursor).unwrap();
    let mut migrated_sim = Simulation::from_snapshot(migrated_v3);

    // Move player to Inn (LocationId 1) where Mira is
    migrated_sim.push_action(PlayerAction::Move { to: LocationId(1) });
    migrated_sim.step();
    migrated_sim.drain_results();

    // Set canonical bond on Mira Ashbridge (CitizenId 1) to DevotedAlly
    {
        let mut query = migrated_sim
            .world
            .query::<(&CitizenMeta, &mut RelationalLedger)>();
        for (meta, mut ledger) in query.iter_mut(&mut migrated_sim.world) {
            if meta.id == CitizenId(1) {
                ledger.set_bond(
                    CitizenId::PLAYER,
                    RelationalBond {
                        sentiment: 80,
                        trust: 80,
                        obligation: 0,
                    },
                );
            }
        }
    }

    // Verify Alden behaves strictly as DevotedAlly despite legacy score of -95
    migrated_sim.push_action(PlayerAction::Talk {
        npc: CitizenId(1),
        topic: TalkTopic::RequestWork,
    });
    migrated_sim.step();
    let res = migrated_sim.drain_results();
    assert!(
        res[0]
            .message
            .contains("Always a pleasure to work beside you, my friend."),
        "DevotedAlly must govern; legacy negative score must have 0 effect: {}",
        res[0].message
    );
}

// ── 9. AC-206 Complete Causal Trace Lineage ───────────────────────────────────

#[test]
fn test_ac206_causal_trace_lineage() {
    let mut sim = Simulation::new();

    // Move to Forest Edge (11)
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

    // 1. Root Action
    sim.push_action(PlayerAction::HelpWithFelling { npc: CitizenId(6) });
    sim.step();
    assert!(sim.drain_results()[0].success);

    // 2. Root Event ID verification in EventRing
    let ring = sim.world.resource::<EventRing>();
    let causal_event = ring
        .events
        .iter()
        .find(|e| matches!(e, SimEvent::CausalAction { action_name, .. } if action_name == "HelpWithFelling"))
        .expect("CausalAction event must exist");

    let root_id = match causal_event {
        SimEvent::CausalAction { causal, .. } => causal.root_event_id,
        _ => unreachable!(),
    };

    // 3. Consequence registry tracks root_id
    {
        let reg = sim.world.resource::<PendingConsequenceRegistry>();
        assert_eq!(reg.consequences.len(), 1);
        let cons = &reg.consequences[0];
        assert_eq!(
            cons.causal_root, root_id,
            "Consequence must link to root action ID"
        );
        assert_eq!(
            cons.stage,
            ConsequenceStage::Active,
            "Stage must be Active initially"
        );
    }

    // 4. Advance 14 days to mature consequence
    sim.advance(336);

    // 5. Consequence matured
    {
        let reg = sim.world.resource::<PendingConsequenceRegistry>();
        assert_eq!(reg.consequences[0].stage, ConsequenceStage::Matured);
    }

    // 6. Observable state transformation: Runn Birch transformed to Artisan at Forge (LocationId 2)
    {
        let mut query = sim
            .world
            .query::<(&CitizenMeta, &NpcSchedule, &OccupationProfile)>();
        for (meta, sched, occ) in query.iter(&sim.world) {
            if meta.id == CitizenId(12) {
                assert_eq!(occ.occupation, OccupationType::Artisan);
                assert_eq!(sched.work_location, LocationId(2));
            }
        }
    }
}
