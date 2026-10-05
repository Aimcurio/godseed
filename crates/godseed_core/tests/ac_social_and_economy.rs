use godseed_core::{
    components::{CitizenMeta, RelationalLedger},
    settlement::SettlementDirectory,
    sim::Simulation,
    types::{CitizenId, LocationId, OccupationType, PlayerAction, ResourceType, TalkTopic},
};

fn player_bond_score(sim: &mut Simulation, npc: CitizenId) -> i16 {
    let mut q = sim.world.query::<(&CitizenMeta, &RelationalLedger)>();
    let (_, ledger) = q
        .iter(&sim.world)
        .find(|(meta, _)| meta.id == npc)
        .expect("NPC should have canonical VS2 relational ledger");
    let bond = ledger.get_bond(CitizenId::PLAYER);
    ((bond.sentiment as i16 + bond.trust as i16) / 2).clamp(-100, 100)
}

#[test]
fn test_ac4_dialogue_and_social_relationship() {
    let mut sim = Simulation::new();

    // Player starts at Road (Loc 9). Mira is at Inn (Loc 1) at hour 0.
    // Move to Inn
    sim.push_action(PlayerAction::Move { to: LocationId(1) });
    sim.step();
    let move_res = sim.drain_results();
    assert!(move_res[0].success, "Move to inn should succeed");

    // Talk to Mira
    let initial_rel = player_bond_score(&mut sim, CitizenId(1));

    sim.push_action(PlayerAction::Talk {
        npc: CitizenId(1),
        topic: TalkTopic::Greeting,
    });
    sim.step();
    let talk_res = sim.drain_results();
    assert!(talk_res[0].success, "Talk to Mira should succeed");
    assert!(
        talk_res[0].message.contains("Mira"),
        "Response should mention Mira"
    );

    let post_rel = player_bond_score(&mut sim, CitizenId(1));

    assert!(
        post_rel > initial_rel,
        "Greeting should increase relationship with Mira (was {}, now {})",
        initial_rel,
        post_rel
    );
}

#[test]
fn test_ac5_and_ac8_economy_and_settlement_alteration() {
    let mut sim = Simulation::new();
    let sid = SettlementDirectory::thornveil_id();

    // Check initial stock and player coins
    let initial_food_stock = {
        let dir = sim.world.resource::<SettlementDirectory>();
        dir.get(sid).unwrap().get_stock(0)
    };
    let initial_coins = sim.summary().player.unwrap().coins;
    assert_eq!(initial_coins, 5.0);

    // Buy 2 units of Food
    sim.push_action(PlayerAction::Buy {
        resource: ResourceType::Food,
        quantity: 2,
    });
    sim.step();
    let buy_res = sim.drain_results();
    assert!(buy_res[0].success, "Buy food should succeed");

    let post_buy_coins = sim.summary().player.unwrap().coins;
    assert!(
        post_buy_coins < initial_coins,
        "Coins must decrease after buying"
    );

    let post_buy_stock = {
        let dir = sim.world.resource::<SettlementDirectory>();
        dir.get(sid).unwrap().get_stock(0)
    };
    assert_eq!(
        post_buy_stock,
        initial_food_stock - 2.0,
        "Settlement food stock must decrease by purchased quantity"
    );

    // Work as a laborer to alter settlement and earn coins
    sim.push_action(PlayerAction::Work {
        occupation: OccupationType::Laborer,
    });
    sim.step();
    let work_res = sim.drain_results();
    assert!(work_res[0].success, "Laborer work should succeed");

    let post_work_coins = sim.summary().player.unwrap().coins;
    assert!(
        post_work_coins > post_buy_coins,
        "Coins must increase after working"
    );
}

#[test]
fn test_ac13_gossip_and_social_memory() {
    let mut sim = Simulation::new();

    // Step into Inn and build a high relationship with Mira
    sim.push_action(PlayerAction::Move { to: LocationId(1) });
    sim.step();
    sim.drain_results();

    for _ in 0..5 {
        sim.push_action(PlayerAction::Talk {
            npc: CitizenId(1),
            topic: TalkTopic::Greeting,
        });
        sim.step();
        sim.drain_results();
    }

    let mira_rel = player_bond_score(&mut sim, CitizenId(1));
    assert!(mira_rel > 0, "Mira relationship should be positive");

    // Advance 7 ticks (a weekly cycle runs at tick % 7 == 0)
    sim.advance(7);

    // Invariants must hold after gossip propagation
    assert!(sim.check_invariants().is_ok());
}
