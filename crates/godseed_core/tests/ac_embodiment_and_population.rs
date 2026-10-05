use godseed_core::{
    components::{
        CausalAudit, CitizenMeta, Demographics, HouseholdRef, Inventory, Kinship, MobilityProfile,
        OccupationProfile, PersonalFinances, PhysicalNeeds, PlayerMarker, SettlementRef,
    },
    sim::Simulation,
    types::{CitizenId, LocationId, OccupationType, PlayerAction},
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
    assert!(
        sim.check_invariants().is_ok(),
        "Initial state must satisfy all invariants"
    );
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

#[test]
fn test_player_as_citizen_invariants_and_metabolic_parity() {
    let mut sim = Simulation::new();

    // 1. Component Verification: Verify Player entity possesses all core citizen components
    {
        let mut player_query = sim.world.query::<(
            &PlayerMarker,
            &CitizenMeta,
            &Demographics,
            &HouseholdRef,
            &SettlementRef,
            &OccupationProfile,
            &PersonalFinances,
            &PhysicalNeeds,
            &MobilityProfile,
            &Kinship,
            &CausalAudit,
            &Inventory,
        )>();

        let count = player_query.iter(&sim.world).count();
        assert_eq!(
            count, 1,
            "Player must possess all 11 core citizen components"
        );
    }

    // 2. Metabolic Decay Parity: Verify player and NPC decay at identical rate
    {
        // Align initial satiety, rest, and health for player (Citizen 0) and NPC (Citizen 1)
        let mut needs_query = sim
            .world
            .query::<(&CitizenMeta, &mut PhysicalNeeds, &mut Demographics)>();
        for (meta, mut needs, mut demo) in needs_query.iter_mut(&mut sim.world) {
            if meta.id == CitizenId(0) || meta.id == CitizenId(1) {
                needs.satiety = 80;
                needs.rest = 80;
                demo.health = 100;
            }
        }

        // Advance 6 ticks. At tick 6, clock.tick % 6 == 0 fires once, decaying satiety by 1 for both.
        sim.advance(6);

        let mut p_satiety = 0;
        let mut npc_satiety = 0;
        let mut needs_check = sim.world.query::<(&CitizenMeta, &PhysicalNeeds)>();
        for (meta, needs) in needs_check.iter(&sim.world) {
            if meta.id == CitizenId(0) {
                p_satiety = needs.satiety;
            } else if meta.id == CitizenId(1) {
                npc_satiety = needs.satiety;
            }
        }

        assert_eq!(p_satiety, 79, "Player satiety must decay by 1 over 6 ticks");
        assert_eq!(npc_satiety, 79, "NPC satiety must decay by 1 over 6 ticks");
        assert_eq!(
            p_satiety, npc_satiety,
            "Player and NPC must obey identical metabolic decay rate"
        );
    }

    // 3. Starvation Health Collapse Parity: When satiety = 0, health decays at 2/tick until death
    {
        let mut sim_starve = Simulation::new();
        // Clear food stock so NPCs cannot eat from settlement stockpile during starvation test
        {
            let mut settlements = sim_starve
                .world
                .resource_mut::<godseed_core::settlement::SettlementDirectory>();
            if let Some(s) =
                settlements.get_mut(godseed_core::settlement::SettlementDirectory::thornveil_id())
            {
                s.resource_stockpile.insert(0, 0.0);
            }
        }
        let mut needs_query = sim_starve
            .world
            .query::<(&CitizenMeta, &mut PhysicalNeeds, &mut Demographics)>();
        for (meta, mut needs, mut demo) in needs_query.iter_mut(&mut sim_starve.world) {
            if meta.id == CitizenId(0) || meta.id == CitizenId(1) {
                needs.satiety = 0;
                demo.health = 20;
            }
        }

        // Advance 5 ticks
        sim_starve.advance(5);

        let mut p_health = 0;
        let mut npc_health = 0;
        let mut check_query = sim_starve.world.query::<(&CitizenMeta, &Demographics)>();
        for (meta, demo) in check_query.iter(&sim_starve.world) {
            if meta.id == CitizenId(0) {
                p_health = demo.health;
            } else if meta.id == CitizenId(1) {
                npc_health = demo.health;
            }
        }

        assert_eq!(
            p_health, 10,
            "Player health must collapse by 2 per starving tick (20 - 10 = 10)"
        );
        assert_eq!(
            npc_health, 10,
            "NPC health must collapse at identical rate (20 - 10 = 10)"
        );
        assert_eq!(
            p_health, npc_health,
            "Starvation health collapse rate must be identical"
        );

        // Test death transition: set health to 2 with satiety 0, advance 1 tick
        for (meta, mut needs, mut demo) in needs_query.iter_mut(&mut sim_starve.world) {
            if meta.id == CitizenId(0) || meta.id == CitizenId(1) {
                needs.satiety = 0;
                demo.health = 2;
            }
        }
        sim_starve.advance(1);

        let mut p_alive = true;
        let mut npc_alive = true;
        let mut alive_query = sim_starve.world.query::<&CitizenMeta>();
        for meta in alive_query.iter(&sim_starve.world) {
            if meta.id == CitizenId(0) {
                p_alive = meta.alive;
            } else if meta.id == CitizenId(1) {
                npc_alive = meta.alive;
            }
        }
        assert!(!p_alive, "Player must perish from starvation at 0 health");
        assert!(!npc_alive, "NPC must perish from starvation at 0 health");
    }

    // 4. Labor Wage & Rest Cost: Player engages in labor subject to standard economy
    {
        let mut sim_work = Simulation::new();
        let initial_summary = sim_work.summary();
        let initial_player = initial_summary.player.expect("Player exists");
        let initial_coins = initial_player.coins;
        let initial_rest = initial_player.rest;

        sim_work.push_action(PlayerAction::Work {
            occupation: OccupationType::Laborer,
        });
        sim_work.step();
        let results = sim_work.drain_results();

        assert!(!results.is_empty());
        assert!(results[0].success, "Laborer work must succeed");

        let after_summary = sim_work.summary();
        let after_player = after_summary.player.expect("Player exists");

        // Wage earned: +1.5 coins
        assert_eq!(
            after_player.coins,
            initial_coins + 1.5,
            "Work must pay standard wage"
        );
        // Rest expended: -10 rest
        assert_eq!(
            after_player.rest,
            initial_rest.saturating_sub(10),
            "Work must expend rest"
        );
    }
}
