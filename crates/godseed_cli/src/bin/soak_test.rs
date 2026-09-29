use godseed_core::sim::Simulation;
use std::time::Instant;

fn main() {
    println!("[SOAK] Starting 2,160-tick long-horizon soak test (90 simulated days)...");
    let start = Instant::now();
    let mut sim = Simulation::new();

    let mut checkpoints = Vec::new();
    let total_ticks = 2160;
    let checkpoint_interval = 240;

    for i in 0..(total_ticks / checkpoint_interval) {
        let step_start = Instant::now();
        sim.advance(checkpoint_interval);
        let step_duration = step_start.elapsed();

        let inv_res = sim.check_invariants();
        let is_ok = inv_res.is_ok();
        let summary = sim.summary();
        let hash = sim.state_hash();

        println!(
            "[SOAK CHECKPOINT {}] Day {:02} | Hash: {:016x} | Invariants: {} | Living NPCs: {} | Duration: {:?}",
            i + 1,
            summary.day,
            hash,
            if is_ok { "PASS" } else { "FAIL" },
            summary.living_npcs,
            step_duration
        );

        checkpoints.push(format!(
            "{{\"day\": {}, \"tick\": {}, \"hash\": \"{:016x}\", \"invariants_pass\": {}, \"living_npcs\": {}}}",
            summary.day, summary.tick, hash, is_ok, summary.living_npcs
        ));

        if !is_ok {
            eprintln!("Invariant failure at checkpoint {}: {:?}", i + 1, inv_res);
            std::process::exit(1);
        }
    }

    let total_duration = start.elapsed();
    let tps = (total_ticks as f64) / total_duration.as_secs_f64();
    println!("[SOAK] COMPLETED {} ticks in {:?} ({:.0} ticks/sec)", total_ticks, total_duration, tps);

    let json = format!(
        "{{\n  \"total_ticks\": {},\n  \"duration_ms\": {},\n  \"ticks_per_sec\": {:.1},\n  \"checkpoints\": [\n    {}\n  ]\n}}\n",
        total_ticks, total_duration.as_millis(), tps, checkpoints.join(",\n    ")
    );
    std::fs::write("evidence/soak_test_report.json", json).expect("Write soak report");
    println!("[SOAK] Evidence written to evidence/soak_test_report.json");
}
