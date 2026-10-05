use godseed_core::{
    content::{caps, knowledge},
    sim::Simulation,
    types::{CapabilityLevel, LocationId, PlayerAction},
};

#[test]
fn test_ac6_capability_acquisition_and_progression() {
    let mut sim = Simulation::new();

    // Player starts with no Woodcutting capability
    {
        use godseed_core::components::{CapabilitySet, PlayerMarker};
        let mut q = sim
            .world
            .query_filtered::<&CapabilitySet, bevy_ecs::query::With<PlayerMarker>>();
        let caps = q.iter(&sim.world).next().unwrap();
        assert!(!caps.has(caps::WOODCUTTING));
    }

    // Practice Woodcutting (starts as novice)
    sim.push_action(PlayerAction::Practice {
        capability: caps::WOODCUTTING,
    });
    sim.step();
    let res = sim.drain_results();
    assert!(res[0].success, "Practice should succeed");

    // Player now has Woodcutting at Novice level
    {
        use godseed_core::components::{CapabilitySet, PlayerMarker};
        let mut q = sim
            .world
            .query_filtered::<&CapabilitySet, bevy_ecs::query::With<PlayerMarker>>();
        let caps = q.iter(&sim.world).next().unwrap();
        assert!(caps.has(caps::WOODCUTTING));
        assert_eq!(caps.level(caps::WOODCUTTING), CapabilityLevel::NOVICE);
    }
}

#[test]
fn test_ac7_transformation_path_the_inscription_path() {
    let mut sim = Simulation::new();

    // Step 1: Discover the Old Archive (Location 8)
    // Road (9) is connected to Well (7), Well (7) is connected to Archive (8)
    sim.push_action(PlayerAction::Move { to: LocationId(7) });
    sim.step();
    sim.drain_results();

    sim.push_action(PlayerAction::Move { to: LocationId(8) });
    sim.step();
    sim.drain_results();

    // Study Archive to gain Ancient Archive knowledge node
    sim.push_action(PlayerAction::StudyArchive);
    sim.step();
    let study_res = sim.drain_results();
    assert!(study_res[0].success);

    // Verify knowledge gained
    {
        use godseed_core::components::{KnowledgeInventory, PlayerMarker};
        let mut q = sim
            .world
            .query_filtered::<&KnowledgeInventory, bevy_ecs::query::With<PlayerMarker>>();
        let k = q.iter(&sim.world).next().unwrap();
        assert!(
            k.knows(knowledge::ANCIENT_ARCHIVE),
            "Must know about Ancient Archive"
        );
    }

    // Step 2: Practice/Acquire Inscription capability
    sim.push_action(PlayerAction::Practice {
        capability: caps::INSCRIPTION,
    });
    sim.step();
    sim.drain_results();

    // Step 3: Advance 30 ticks to trigger monthly transformation system
    sim.advance(30);

    let summary = sim.summary();
    let player = summary.player.unwrap();
    assert_eq!(
        player.transformation_stage, 1,
        "Player should advance to Stage 1 (Scholar) upon acquiring Inscription"
    );

    // Step 4: Make 5 inscriptions
    for i in 1..=5 {
        sim.push_action(PlayerAction::Inscribe {
            observation: format!(
                "Observation #{}: Recording settlement life in Thornveil.",
                i
            ),
        });
        sim.step();
        let inscribe_res = sim.drain_results();
        assert!(inscribe_res[0].success);
    }

    let summary_after_inscribe = sim.summary();
    let player_after_inscribe = summary_after_inscribe.player.unwrap();
    assert_eq!(
        player_after_inscribe.inscriptions, 5,
        "Player must have 5 completed inscriptions"
    );

    // Run monthly check
    sim.advance(30);
    let summary_final = sim.summary();
    let player_final = summary_final.player.unwrap();
    assert!(
        player_final.transformation_progress >= 30,
        "Progress must advance after completing 5 inscriptions"
    );
}
