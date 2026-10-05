use godseed_core::{
    components::{CitizenMeta, EpistemicState},
    events::SimEvent,
    resources::EventRing,
    sim::Simulation,
    types::{CitizenId, KnowledgeNodeId, LocationId, PlayerAction, TalkTopic},
};

#[test]
fn test_epistemic_state_and_corroborating_gossip() {
    let mut sim = Simulation::new();

    // 1. Initial State: Verify initial knowledge distribution
    {
        let mut query = sim.world.query::<(&CitizenMeta, &EpistemicState)>();
        let mut tomas_knows_timber = false;
        let mut voss_knows_exile = false;

        for (meta, epistemic) in query.iter(&sim.world) {
            if meta.id == CitizenId(6) {
                tomas_knows_timber = epistemic.has_knowledge(1);
                assert_eq!(epistemic.get_corroboration(1), 1);
            }
            if meta.id == CitizenId(5) {
                voss_knows_exile = epistemic.has_knowledge(5);
                assert!(epistemic.has_knowledge(4));
                assert!(epistemic.has_knowledge(7));
            }
        }
        assert!(
            tomas_knows_timber,
            "Tomas Birch must start with Timber Stress knowledge (ID 1)"
        );
        assert!(
            voss_knows_exile,
            "Elder Voss must start with Exiled Son knowledge (ID 5)"
        );
    }

    // 2. Direct Player Discovery: Player learns from Forester Tomas Birch
    // Move to Forest Edge (11) via 9 -> 7 -> 8 -> 11
    sim.push_action(PlayerAction::Move { to: LocationId(7) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(8) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(11) });
    sim.step();

    // Wait until working hours (hour 8)
    while sim.summary().hour < 8 {
        sim.push_action(PlayerAction::Wait { ticks: 1 });
        sim.step();
        sim.drain_results();
    }

    // Ask Tomas about work
    sim.push_action(PlayerAction::Talk {
        npc: CitizenId(6),
        topic: TalkTopic::AskAboutWork,
    });
    sim.step();
    let talk_res = sim.drain_results();
    assert!(talk_res[0].success);

    // Verify Player has learned Knowledge ID 1 (Timber Stress)
    {
        let mut query = sim.world.query_filtered::<&EpistemicState, bevy_ecs::prelude::With<godseed_core::components::PlayerMarker>>();
        let player_epistemic = query
            .iter(&sim.world)
            .next()
            .expect("Player must have EpistemicState");
        assert!(
            player_epistemic.has_knowledge(1),
            "Player must acquire Timber Stress (ID 1) after discussing forestry work"
        );
        assert_eq!(player_epistemic.get_corroboration(1), 1);
    }

    // 3. Player shares knowledge with another citizen:
    // Move to The Slanted Timber Inn (Location 1) to find Mira Ashbridge (CitizenId 1)
    // 11 -> 9 -> 1
    sim.push_action(PlayerAction::Move { to: LocationId(9) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(1) });
    sim.step();

    // Wait until Mira is at work (hour 14)
    while sim.summary().hour != 14 {
        sim.push_action(PlayerAction::Wait { ticks: 1 });
        sim.step();
        sim.drain_results();
    }

    // Share Knowledge ID 1 with Mira
    sim.push_action(PlayerAction::Talk {
        npc: CitizenId(1),
        topic: TalkTopic::ShareKnowledge {
            node: KnowledgeNodeId(1),
        },
    });
    sim.step();
    assert!(sim.drain_results()[0].success);

    // Verify Mira Ashbridge now holds Knowledge ID 1 with corroboration count = 1
    {
        let mut query = sim.world.query::<(&CitizenMeta, &EpistemicState)>();
        let mut mira_learned = false;
        for (meta, epistemic) in query.iter(&sim.world) {
            if meta.id == CitizenId(1) {
                mira_learned = epistemic.has_knowledge(1);
                assert_eq!(epistemic.get_corroboration(1), 1);
            }
        }
        assert!(
            mira_learned,
            "Mira Ashbridge must have acquired Knowledge ID 1 from player"
        );
    }

    // 4. Asymmetric Autonomous Gossip & Corroboration Cycle:
    // Test direct social transfer between Elder Voss (CitizenId 5) and Aldous Minner (CitizenId 10).
    // Both socialize at The Well (Location 7) at hour 17.
    // Create an isolated sub-scenario at hour 17 to precisely trace 1 -> 2 -> 3 -> no-op suppression.
    let mut gossip_sim = Simulation::new();

    // Advance to hour 17 on day 0
    for _ in 0..17 {
        gossip_sim.push_action(PlayerAction::Wait { ticks: 1 });
        gossip_sim.step();
        gossip_sim.drain_results();
    }
    assert_eq!(gossip_sim.summary().hour, 17);

    // Initial check: Aldous Minner starts with 0 knowledge
    {
        let mut query = gossip_sim.world.query::<(&CitizenMeta, &EpistemicState)>();
        for (meta, epistemic) in query.iter(&gossip_sim.world) {
            if meta.id == CitizenId(10) {
                assert_eq!(
                    epistemic.known.len(),
                    0,
                    "Aldous must start with 0 knowledge nodes"
                );
            }
        }
    }

    // Step 1: Execute weekly gossip schedule once
    gossip_sim.step_weekly();

    // Verify Novel Transfer (Corroboration Count = 1)
    {
        let mut query = gossip_sim.world.query::<(&CitizenMeta, &EpistemicState)>();
        for (meta, epistemic) in query.iter(&gossip_sim.world) {
            if meta.id == CitizenId(10) {
                assert!(
                    !epistemic.known.is_empty(),
                    "Aldous must have received knowledge from Voss"
                );
                for (_, corr) in epistemic.known.values() {
                    assert_eq!(*corr, 1, "Novel transfer must establish corroboration = 1");
                }
            }
        }
    }

    // Step 2: Second gossip transmission
    gossip_sim.step_weekly();

    // Verify Corroborating Transfer (Corroboration Count = 2)
    {
        let mut query = gossip_sim.world.query::<(&CitizenMeta, &EpistemicState)>();
        for (meta, epistemic) in query.iter(&gossip_sim.world) {
            if meta.id == CitizenId(10) {
                for (_, corr) in epistemic.known.values() {
                    assert_eq!(
                        *corr, 2,
                        "Second transfer must increment corroboration to 2"
                    );
                }
            }
        }
    }

    // Step 3: Third gossip transmission (Saturation at 3)
    gossip_sim.step_weekly();

    {
        let mut query = gossip_sim.world.query::<(&CitizenMeta, &EpistemicState)>();
        for (meta, epistemic) in query.iter(&gossip_sim.world) {
            if meta.id == CitizenId(10) {
                for (_, corr) in epistemic.known.values() {
                    assert_eq!(*corr, 3, "Third transfer must reach saturation at 3");
                }
            }
        }
    }

    // Record event count before fourth transmission
    let events_before = {
        let ring = gossip_sim.world.resource::<EventRing>();
        ring.events
            .iter()
            .filter(|e| matches!(e, SimEvent::KnowledgeShared { .. }))
            .count()
    };

    // Step 4: Fourth gossip transmission — should trigger No-Op Suppression (AC-204)
    gossip_sim.step_weekly();

    let events_after = {
        let ring = gossip_sim.world.resource::<EventRing>();
        ring.events
            .iter()
            .filter(|e| matches!(e, SimEvent::KnowledgeShared { .. }))
            .count()
    };

    assert_eq!(
        events_before, events_after,
        "No-Op Suppression: Saturated knowledge (corroboration >= 3) must NOT emit new events or mutate state"
    );

    // 5. Invariant Verification: All machine-checkable invariants must pass
    assert!(
        gossip_sim.check_invariants().is_ok(),
        "All invariants (INV-1 through INV-7) must pass: {:?}",
        gossip_sim.check_invariants()
    );
    assert!(
        sim.check_invariants().is_ok(),
        "Main sim invariants must also pass: {:?}",
        sim.check_invariants()
    );
}
