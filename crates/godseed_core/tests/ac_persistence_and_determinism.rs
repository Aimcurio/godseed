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
fn test_ac9_deep_semantic_persistence_equivalence() {
    let mut sim = Simulation::new();

    // Advance 48 ticks to allow autonomous routine and economic churn
    sim.advance(48);

    // Perform a series of interactive player actions across multiple locations
    // 1. Move to Old Archive (loc 6)
    sim.push_action(PlayerAction::Move { to: LocationId(6) });
    sim.step();
    sim.drain_results();

    // 2. Study the archive
    sim.push_action(PlayerAction::StudyArchive);
    sim.step();
    sim.drain_results();

    // 3. Practice inscription
    sim.push_action(PlayerAction::Practice { capability: godseed_core::types::CapabilityId(1) });
    sim.step();
    sim.drain_results();

    // 4. Move to Market Square (loc 3)
    sim.push_action(PlayerAction::Move { to: LocationId(3) });
    sim.step();
    sim.drain_results();

    // 5. Buy food
    sim.push_action(PlayerAction::Buy {
        resource: godseed_core::types::ResourceType::Food,
        quantity: 1,
    });
    sim.step();
    sim.drain_results();

    // Advance 12 more ticks so NPCs update memory and gossip
    sim.advance(12);

    // Build pre-save snapshot
    let pre_save = sim.build_snapshot();
    let pre_save_hash = sim.state_hash();

    // Save snapshot to file
    let save_path = "saves/test_semantic_equivalence.gs1";
    fs::create_dir_all("saves").unwrap();
    sim.save(save_path).expect("Save must succeed");

    // Load into new simulation
    let mut loaded_sim = Simulation::load_from_file(save_path).expect("Load must succeed");
    let loaded = loaded_sim.build_snapshot();
    let loaded_hash = loaded_sim.state_hash();

    // Assert overall snapshot deep equality
    assert_eq!(pre_save_hash, loaded_hash, "State hash must match identically");
    assert_eq!(pre_save, loaded, "Full SimulationSnapshot must be deeply identical");

    // Explicit field-by-field verification (Section 8: detect missing-component loss or default substitution)
    assert_eq!(pre_save.version, loaded.version, "Version must match");
    assert_eq!(pre_save.clock, loaded.clock, "SimClock must match");
    assert_eq!(pre_save.world_map, loaded.world_map, "WorldMap must match");
    assert_eq!(pre_save.settlements, loaded.settlements, "SettlementDirectory must match");
    assert_eq!(pre_save.households, loaded.households, "HouseholdDirectory must match");
    assert_eq!(pre_save.relationships, loaded.relationships, "RelationshipLedger must match");
    assert_eq!(pre_save.reputation, loaded.reputation, "ReputationRegistry must match");
    assert_eq!(pre_save.events, loaded.events, "EventRing must match");
    assert_eq!(pre_save.next_citizen_id, loaded.next_citizen_id, "NextCitizenId must match");
    assert_eq!(pre_save.citizens.len(), loaded.citizens.len(), "Citizen counts must match exactly");

    // Verify all 16 citizens (15 NPCs + 1 player)
    assert_eq!(pre_save.citizens.len(), 16);
    for pre_c in &pre_save.citizens {
        let loaded_c = loaded.citizens.iter()
            .find(|c| c.meta.id == pre_c.meta.id)
            .unwrap_or_else(|| panic!("Citizen {:?} missing from loaded snapshot", pre_c.meta.id));

        assert_eq!(&pre_c.meta, &loaded_c.meta, "CitizenMeta mismatch for {:?}", pre_c.meta.id);
        assert_eq!(&pre_c.demographics, &loaded_c.demographics, "Demographics mismatch for {:?}", pre_c.meta.id);
        assert_eq!(&pre_c.household_ref, &loaded_c.household_ref, "HouseholdRef mismatch for {:?}", pre_c.meta.id);
        assert_eq!(&pre_c.settlement_ref, &loaded_c.settlement_ref, "SettlementRef mismatch for {:?}", pre_c.meta.id);
        assert_eq!(&pre_c.occupation, &loaded_c.occupation, "OccupationProfile mismatch for {:?}", pre_c.meta.id);
        assert_eq!(&pre_c.finances, &loaded_c.finances, "PersonalFinances mismatch for {:?}", pre_c.meta.id);
        assert_eq!(&pre_c.needs, &loaded_c.needs, "PhysicalNeeds mismatch for {:?}", pre_c.meta.id);
        assert_eq!(&pre_c.mobility, &loaded_c.mobility, "MobilityProfile mismatch for {:?}", pre_c.meta.id);
        assert_eq!(&pre_c.kinship, &loaded_c.kinship, "Kinship mismatch for {:?}", pre_c.meta.id);
        assert_eq!(&pre_c.causal_audit, &loaded_c.causal_audit, "CausalAudit mismatch for {:?}", pre_c.meta.id);
        assert_eq!(&pre_c.inventory, &loaded_c.inventory, "Inventory mismatch for {:?}", pre_c.meta.id);

        if pre_c.is_player {
            assert!(loaded_c.is_player, "Loaded entity must be marked as player");
            assert_eq!(&pre_c.capabilities, &loaded_c.capabilities, "Player capabilities mismatch");
            assert_eq!(&pre_c.transformation, &loaded_c.transformation, "Player transformation mismatch");
            assert_eq!(&pre_c.knowledge, &loaded_c.knowledge, "Player knowledge mismatch");
            assert!(loaded_c.capabilities.is_some(), "Player capabilities must not be lost or None");
            assert!(loaded_c.transformation.is_some(), "Player transformation must not be lost or None");
            assert!(loaded_c.knowledge.is_some(), "Player knowledge must not be lost or None");
        } else {
            assert_eq!(&pre_c.npc_memory, &loaded_c.npc_memory, "NPC memory mismatch for {:?}", pre_c.meta.id);
            assert_eq!(&pre_c.npc_schedule, &loaded_c.npc_schedule, "NPC schedule mismatch for {:?}", pre_c.meta.id);
            assert_eq!(&pre_c.npc_goals, &loaded_c.npc_goals, "NPC goals mismatch for {:?}", pre_c.meta.id);
            assert_eq!(&pre_c.disposition, &loaded_c.disposition, "NPC disposition mismatch for {:?}", pre_c.meta.id);
            assert!(loaded_c.npc_schedule.is_some(), "NPC schedule must not be lost or None");
            assert!(loaded_c.disposition.is_some(), "NPC disposition must not be lost or None");
        }
    }

    // Verify invariants pass on loaded simulation
    assert!(loaded_sim.check_invariants().is_ok(), "Loaded simulation must pass all invariants");

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
