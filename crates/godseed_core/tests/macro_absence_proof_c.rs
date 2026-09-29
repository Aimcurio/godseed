use godseed_core::{
    components::{CitizenMeta, NpcSchedule, OccupationProfile},
    resources::{PendingConsequenceRegistry, ReturnDigestLog},
    settlement::SettlementDirectory,
    sim::Simulation,
    types::{CitizenId, ConsequenceStage, LocationId, OccupationType, PlayerAction, TalkTopic},
};

#[test]
fn test_proof_c_macro_absence_and_return_digest() {
    let mut sim = Simulation::new();
    let sid = SettlementDirectory::thornveil_id();

    // 1. Initial State: Check initial timber price (Resource ordinal 1 = Timber, base 5.0)
    let initial_timber_price = {
        let settlements = sim.world.resource::<SettlementDirectory>();
        settlements.get(sid).expect("Thornveil must exist").get_price(1)
    };
    assert!((initial_timber_price - 5.0).abs() < 0.1, "Initial timber price must be ~5.0 coins");

    // 2. Player initiates causal felling deed at West Woods (Location 11) with Tomas Birch (6)
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

    // Verify consequence is registered as Active
    {
        let reg = sim.world.resource::<PendingConsequenceRegistry>();
        assert_eq!(reg.consequences.len(), 1);
        assert_eq!(reg.consequences[0].stage, ConsequenceStage::Active);
    }

    // Move player back to The Slanted Timber Inn (Location 1) before departure/sleep
    sim.push_action(PlayerAction::Move { to: LocationId(9) });
    sim.step();
    sim.push_action(PlayerAction::Move { to: LocationId(1) });
    sim.step();
    sim.drain_results();

    // 3. Player departs / fast-forwards 30 days (720 ticks) of macro-absence
    sim.advance(720);

    // 4. Verify Consequence Maturation during absence
    {
        let reg = sim.world.resource::<PendingConsequenceRegistry>();
        assert_eq!(reg.consequences[0].stage, ConsequenceStage::Matured, "Consequence must be Matured after 30 days");
    }

    // 5. Verify Runn is now an Artisan working at Forge (Location 2)
    {
        let mut query = sim.world.query::<(&CitizenMeta, &NpcSchedule, &OccupationProfile)>();
        let mut found_runn = false;
        for (meta, sched, occ) in query.iter(&sim.world) {
            if meta.id == CitizenId(12) {
                found_runn = true;
                assert_eq!(occ.occupation, OccupationType::Artisan);
                assert_eq!(sched.work_location, LocationId(2));
            }
        }
        assert!(found_runn);
    }

    // 6. Verify ReturnDigestLog contains digests for both Tomas and Mira
    {
        let digest_log = sim.world.resource::<ReturnDigestLog>();
        assert!(digest_log.find_digest_for(CitizenId(6)).is_some(), "Tomas digest must exist");
        assert!(digest_log.find_digest_for(CitizenId(1)).is_some(), "Mira digest must exist");
    }

    // 7. Player Return Experience: Greet Mira Ashbridge at the Inn (Location 1)
    // Wait until Mira is at work behind the bar (hour 14)
    while sim.summary().hour != 14 {
        sim.push_action(PlayerAction::Wait { ticks: 1 });
        sim.step();
        sim.drain_results();
    }

    sim.push_action(PlayerAction::Talk {
        npc: CitizenId(1),
        topic: TalkTopic::Greeting,
    });
    sim.step();
    let mira_return_res = sim.drain_results();
    assert!(mira_return_res[0].success);
    assert!(
        mira_return_res[0].message.contains("You've been gone a spell")
            && mira_return_res[0].message.contains("Tomas works alone now")
            && mira_return_res[0].message.contains("Timber's gotten dearer"),
        "Mira must deliver epistemic return salutation upon player's return: {}",
        mira_return_res[0].message
    );

    // Verify Mira's return digest was consumed (single-shot delivery)
    {
        let digest_log = sim.world.resource::<ReturnDigestLog>();
        assert!(digest_log.find_digest_for(CitizenId(1)).is_none(), "Mira's digest must be popped once delivered");
    }

    // Subsequent greeting to Mira should return to steady-state consequence greeting
    sim.push_action(PlayerAction::Talk {
        npc: CitizenId(1),
        topic: TalkTopic::Greeting,
    });
    sim.step();
    let mira_second_res = sim.drain_results();
    assert!(mira_second_res[0].success);
    assert!(
        mira_second_res[0].message.contains("holding up at the woodlot"),
        "Second greeting should be steady-state greeting: {}",
        mira_second_res[0].message
    );

    // 8. Economic Impact: Verify Timber Price has risen significantly
    let post_absence_timber_price = {
        let settlements = sim.world.resource::<SettlementDirectory>();
        settlements.get(sid).expect("Thornveil must exist").get_price(1)
    };
    assert!(
        post_absence_timber_price > initial_timber_price,
        "Timber price must have increased due to labor loss (was {:.2}, now {:.2})",
        initial_timber_price, post_absence_timber_price
    );

    // 9. Invariants check
    assert!(sim.check_invariants().is_ok(), "Invariants must hold after 30-day absence: {:?}", sim.check_invariants());
}
