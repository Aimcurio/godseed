use godseed_core::{
    persistence::load_snapshot,
    sim::Simulation,
    types::{CitizenId, LocationId, PlayerAction, ResourceType, TalkTopic},
};

use std::fs;
use std::io::Cursor;

#[test]
fn test_adversarial_invalid_movement() {
    let mut sim = Simulation::new();

    // Player starts at Road (Loc 9). Loc 6 (Herb Garden) is NOT adjacent to Road (Loc 9).
    sim.push_action(PlayerAction::Move { to: LocationId(6) });
    sim.step();
    let res = sim.drain_results();

    assert!(!res.is_empty());
    assert!(
        !res[0].success,
        "Player must not be allowed to move directly to non-adjacent location"
    );
    assert_eq!(
        sim.player_location(),
        LocationId(9),
        "Player position must remain unchanged after rejected move"
    );
}

#[test]
fn test_adversarial_talk_to_nonexistent_or_absent_npc() {
    let mut sim = Simulation::new();

    // Talk to completely non-existent NPC ID
    sim.push_action(PlayerAction::Talk {
        npc: CitizenId(9999),
        topic: TalkTopic::Greeting,
    });
    sim.step();
    let res = sim.drain_results();
    assert!(!res[0].success, "Talking to non-existent NPC must fail");

    // Talk to NPC who is at another location
    // Player is at Road (9); Mira is at Inn (1)
    sim.push_action(PlayerAction::Talk {
        npc: CitizenId(1),
        topic: TalkTopic::Greeting,
    });
    sim.step();
    let res2 = sim.drain_results();
    assert!(!res2[0].success, "Talking to absent NPC must fail");
    assert!(res2[0].message.contains("isn't here"));
}

#[test]
fn test_adversarial_inscribe_without_capability() {
    let mut sim = Simulation::new();

    // Player starts without Inscription capability
    sim.push_action(PlayerAction::Inscribe {
        observation: "Testing unauthorized inscription".to_string(),
    });
    sim.step();
    let res = sim.drain_results();

    assert!(!res[0].success, "Inscribing without capability must fail");
    assert!(res[0].message.contains("learn Inscription first"));
}

#[test]
fn test_adversarial_overspending_coins() {
    let mut sim = Simulation::new();
    let initial_coins = sim.summary().player.unwrap().coins;

    // Player has 5 coins. Attempting to buy 100 Food (cost ~200 coins)
    sim.push_action(PlayerAction::Buy {
        resource: ResourceType::Food,
        quantity: 100,
    });
    sim.step();
    let res = sim.drain_results();

    assert!(
        !res[0].success,
        "Purchasing beyond available coins must fail"
    );
    assert_eq!(
        sim.summary().player.unwrap().coins,
        initial_coins,
        "Coins must not be deducted on failed purchase"
    );
}

#[test]
fn test_adversarial_corrupted_save_file() {
    let mut sim = Simulation::new();
    let save_path = "saves/adversarial_corrupt_test.gs1";
    fs::create_dir_all("saves").unwrap();
    sim.save(save_path).expect("Save must succeed");

    // Read the saved bytes and flip a single byte in the payload
    let mut bytes = fs::read(save_path).expect("Must read saved file");
    assert!(bytes.len() > 20);

    // Corrupt a byte in the middle of the bincode data payload
    let corrupt_idx = bytes.len() / 2;
    bytes[corrupt_idx] ^= 0xFF;

    // Loading from the corrupted bytes via Cursor must fail with CRC32 mismatch
    let mut cursor = Cursor::new(bytes);
    let load_res = load_snapshot(&mut cursor);

    assert!(load_res.is_err(), "Loading corrupted save must return Err");
    let err_str = load_res.err().unwrap().to_string();
    assert!(
        err_str.contains("CRC32 mismatch"),
        "Error message must specify CRC32 mismatch, got: {}",
        err_str
    );

    // Cleanup
    let _ = fs::remove_file(save_path);
}

#[test]
fn test_adversarial_invalid_magic_header() {
    let mut bytes = vec![0u8; 64];
    bytes[0..8].copy_from_slice(b"BADMAGIC");

    let mut cursor = Cursor::new(bytes);
    let load_res = load_snapshot(&mut cursor);

    assert!(load_res.is_err());
    assert!(load_res
        .err()
        .unwrap()
        .to_string()
        .contains("Invalid save file magic"));
}

#[test]
fn test_performance_tick_rate() {
    let mut sim = Simulation::new();
    let start = std::time::Instant::now();
    let ticks = 2400; // 100 game-days

    sim.advance(ticks);

    let duration = start.elapsed();
    let ticks_per_sec = (ticks as f64) / duration.as_secs_f64();

    println!(
        "Performance: {} ticks in {:?} ({:.0} ticks/sec)",
        ticks, duration, ticks_per_sec
    );
    assert!(
        ticks_per_sec > 500.0,
        "Simulation throughput must exceed 500 ticks/sec (measured: {:.0})",
        ticks_per_sec
    );
}
