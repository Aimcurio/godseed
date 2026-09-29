use godseed_core::{
    sim::Simulation,
    types::{CitizenId, LocationId, PlayerAction},
};

#[test]
fn test_ac1_physical_embodiment_and_needs() {
    let mut sim = Simulation::new();
    let initial_summary = sim.summary();
    let player = initial_summary.player.expect("Player must be spawned");

    assert!(player.alive, "Player must start alive");
    assert_eq!(player.satiety, 75, "Player starts with 75% satiety");
    assert_eq!(player.health, 100, "Player starts with 100% health");
    assert_eq!(player.rest, 80, "Player starts with 80% rest");

    // Advance 48 ticks (2 days) without eating
    sim.advance(48);

    let summary_after = sim.summary();
    let player_after = summary_after.player.expect("Player must still exist");

    // Satiety decays over time
    assert!(
        player_after.satiety < player.satiety,
        "Satiety must decay over simulated time (was {}, now {})",
        player.satiety,
        player_after.satiety
    );

    // Sleeping should restore rest
    sim.push_action(PlayerAction::Sleep);
    sim.step();
    let res = sim.drain_results();
    assert!(!res.is_empty(), "Sleeping must produce an action result");
    assert!(res[0].success, "Sleep action must succeed");

    let summary_rested = sim.summary();
    let player_rested = summary_rested.player.expect("Player must exist");
    assert!(
        player_rested.rest >= player_after.rest,
        "Sleep must restore rest (was {}, now {})",
        player_after.rest,
        player_rested.rest
    );
}

#[test]
fn test_ac2_settlement_population() {
    let mut sim = Simulation::new();
    let summary = sim.summary();

    // Thornveil must contain 15 living authored NPCs
    assert_eq!(
        summary.living_npcs, 15,
        "Thornveil must contain 15 living NPCs at launch"
    );

    // Invariants must hold on initial spawn
    assert!(sim.check_invariants().is_ok(), "Initial state must satisfy all invariants");
}

#[test]
fn test_ac3_autonomous_npc_routines() {
    let mut sim = Simulation::new();

    // Mira Ashbridge (Citizen 1) schedule:
    // hour 6: Working at Inn (Location 1)
    // hour 12: At Market (Location 3)
    // hour 22: Sleeping at Inn (Location 1)

    // Advance to hour 6 (tick 6)
    sim.advance(6);
    let hour6_loc = get_npc_location(&mut sim, CitizenId(1));
    assert_eq!(
        hour6_loc,
        LocationId(1),
        "Mira should be at the Inn (loc 1) at hour 6"
    );

    // Advance to hour 12 (tick 12)
    sim.advance(6);
    let hour12_loc = get_npc_location(&mut sim, CitizenId(1));
    assert_eq!(
        hour12_loc,
        LocationId(3),
        "Mira should be at Market Square (loc 3) at hour 12"
    );
}

fn get_npc_location(sim: &mut Simulation, id: CitizenId) -> LocationId {
    use godseed_core::components::{CitizenMeta, SettlementRef};
    let mut q = sim.world.query::<(&CitizenMeta, &SettlementRef)>();
    for (meta, sref) in q.iter(&sim.world) {
        if meta.id == id {
            return sref.current_location;
        }
    }
    panic!("NPC {:?} not found", id);
}
