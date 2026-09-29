use godseed_core::{
    sim::Simulation,
    types::{LocationId, PlayerAction},
};
use std::fs;

#[test]
fn test_ac9_persistence_and_save_load_integrity() {
    let mut sim = Simulation::new();

    // Advance 48 ticks and move player
    sim.advance(24);
    sim.push_action(PlayerAction::Move { to: LocationId(1) });
    sim.step();
    sim.drain_results();

    let hash_before_save = sim.state_hash();
    let tick_before_save = sim.tick();

    // Save to temp file
    let save_path = "saves/test_ac9_save.gs1";
    fs::create_dir_all("saves").unwrap();
    sim.save(save_path).expect("Save must succeed");

    // Load into a new simulation instance
    let mut loaded_sim = Simulation::load_from_file(save_path).expect("Load must succeed");

    assert_eq!(loaded_sim.tick(), tick_before_save, "Ticks must match");
    assert_eq!(loaded_sim.state_hash(), hash_before_save, "State hash must match across save/load");
    assert!(loaded_sim.check_invariants().is_ok(), "Loaded state must pass all invariants");

    // Cleanup
    let _ = fs::remove_file(save_path);
}

#[test]
fn test_ac10_state_determinism() {
    // Two simulations starting with identical conditions and no player input must produce identical state hashes
    let mut sim1 = Simulation::new();
    let mut sim2 = Simulation::new();

    assert_eq!(sim1.state_hash(), sim2.state_hash(), "Initial state hashes must match");

    sim1.advance(120); // 5 days
    sim2.advance(120);

    assert_eq!(
        sim1.state_hash(),
        sim2.state_hash(),
        "Simulations must remain bit-for-bit deterministic over 120 ticks"
    );
}

#[test]
fn test_ac11_departure_and_return_consequences() {
    let mut sim = Simulation::new();

    let initial_hash = sim.state_hash();
    let _initial_summary = sim.summary();

    // Depart / pass 720 ticks (30 in-game days)

    sim.advance(720);

    let later_summary = sim.summary();
    assert_eq!(later_summary.tick, 720);
    assert_eq!(later_summary.day, 30);

    // World must have changed autonomously
    assert_ne!(
        sim.state_hash(),
        initial_hash,
        "World state hash must change after 30 days of autonomous simulation"
    );

    // Living NPCs must have continued living
    assert!(
        later_summary.living_npcs > 0,
        "NPCs must persist through autonomous simulation"
    );

    // Invariants must hold across 720 ticks
    assert!(sim.check_invariants().is_ok());
}

#[test]
fn test_ac12_invariants_suite() {
    let mut sim = Simulation::new();
    assert!(sim.check_invariants().is_ok());

    sim.advance(240);
    assert!(sim.check_invariants().is_ok(), "Invariants must hold after 10 days");
}

#[test]
fn test_ac14_telemetry_emission() {
    let mut sim = Simulation::new();

    // Push an action
    sim.push_action(PlayerAction::Look);
    sim.step();
    sim.advance(24);

    let log = sim.world.resource::<godseed_core::resources::TelemetryLog>();
    let jsonl = log.to_jsonl();

    assert!(!jsonl.is_empty(), "Telemetry log must contain records");
    assert!(jsonl.contains("PlayerAction"), "Telemetry must record PlayerAction");
}
