use godseed_core::{
    components::{EpisodicMemory, EpistemicState},
    resources::PendingConsequenceRegistry,
    sim::Simulation,
    types::{CitizenId, ConsequenceStage, ConsequenceType, LocationId, TriggerCondition},
};
use std::time::Instant;

fn main() {
    println!("[SOAK] Starting 10,000-tick long-horizon soak test (416.7 simulated days)...");
    let start = Instant::now();
    let mut sim = Simulation::new();

    // Register test consequences to verify autonomous consequence lifecycle under extended soak
    {
        let mut cons_reg = sim.world.resource_mut::<PendingConsequenceRegistry>();
        cons_reg.register(
            101,
            TriggerCondition::TimeElapsed {
                duration_ticks: 336,
            }, // 14 days
            ConsequenceType::FraternalLaborStrain {
                elder: CitizenId(6),
                junior: CitizenId(12),
                target_workplace: LocationId(2),
            },
            0,
        );
        cons_reg.register(
            202,
            TriggerCondition::TimeElapsed {
                duration_ticks: 720,
            }, // 30 days
            ConsequenceType::CropBlightDispute {
                farmer_a: CitizenId(3),
                farmer_b: CitizenId(13),
                location: LocationId(5),
            },
            500,
        );
    }

    let mut checkpoints = Vec::new();
    let total_ticks = 10_000;
    let checkpoint_interval = 500;

    for i in 0..(total_ticks / checkpoint_interval) {
        let step_start = Instant::now();
        sim.advance(checkpoint_interval);
        let step_duration = step_start.elapsed();

        let inv_res = sim.check_invariants();
        let is_ok = inv_res.is_ok();
        let summary = sim.summary();
        let hash = sim.state_hash();

        // Query memory bounds and consequence counts
        let (max_transient, max_anchors) = {
            let mut mem_q = sim.world.query::<&EpisodicMemory>();
            let mut mt = 0;
            let mut ma = 0;
            for mem in mem_q.iter(&sim.world) {
                mt = mt.max(mem.transient.len());
                ma = ma.max(mem.anchors.len());
            }
            (mt, ma)
        };

        let max_epistemic = {
            let mut ep_q = sim.world.query::<&EpistemicState>();
            let mut me = 0;
            for ep in ep_q.iter(&sim.world) {
                me = me.max(ep.known.len());
            }
            me
        };

        let (active_cons, matured_cons, resolved_cons) = {
            let cons_reg = sim.world.resource::<PendingConsequenceRegistry>();
            let mut active = 0;
            let mut matured = 0;
            let mut resolved = 0;
            for c in &cons_reg.consequences {
                match c.stage {
                    ConsequenceStage::Active | ConsequenceStage::Escalated => active += 1,
                    ConsequenceStage::Matured => matured += 1,
                    ConsequenceStage::Resolved => resolved += 1,
                }
            }
            (active, matured, resolved)
        };

        println!(
            "[SOAK CHECKPOINT {:02}] Day {:03} | Hash: {:016x} | INV: {} | NPCs: {} | Mem: (t:{}/12, a:{}/6) | Epistemic: {}/32 | Cons: (act:{}, mat:{}, res:{}) | {:?}",
            i + 1,
            summary.day,
            hash,
            if is_ok { "PASS" } else { "FAIL" },
            summary.living_npcs,
            max_transient,
            max_anchors,
            max_epistemic,
            active_cons,
            matured_cons,
            resolved_cons,
            step_duration
        );

        // Strict bound assertions
        assert!(
            is_ok,
            "Invariant failure at checkpoint {}: {:?}",
            i + 1,
            inv_res
        );
        assert!(
            max_transient <= 12,
            "INV-5 FAIL: Transient memory exceeded limit of 12 (got {})",
            max_transient
        );
        assert!(
            max_anchors <= 6,
            "INV-5 FAIL: Anchor memory exceeded limit of 6 (got {})",
            max_anchors
        );
        assert!(
            max_epistemic <= 32,
            "INV-7 FAIL: Epistemic state exceeded limit of 32 (got {})",
            max_epistemic
        );

        checkpoints.push(format!(
            "{{\"day\": {}, \"tick\": {}, \"hash\": \"{:016x}\", \"invariants_pass\": {}, \"living_npcs\": {}, \"max_transient_mem\": {}, \"max_anchor_mem\": {}, \"max_epistemic_nodes\": {}, \"active_consequences\": {}, \"matured_consequences\": {}, \"resolved_consequences\": {}, \"step_duration_us\": {}}}",
            summary.day, summary.tick, hash, is_ok, summary.living_npcs,
            max_transient, max_anchors, max_epistemic,
            active_cons, matured_cons, resolved_cons,
            step_duration.as_micros()
        ));
    }

    let total_duration = start.elapsed();
    let tps = (total_ticks as f64) / total_duration.as_secs_f64();
    println!(
        "[SOAK] COMPLETED {} ticks in {:?} ({:.0} ticks/sec)",
        total_ticks, total_duration, tps
    );

    let json = format!(
        "{{\n  \"campaign\": \"GODSEED_VS2\",\n  \"total_ticks\": {},\n  \"simulated_days\": {:.1},\n  \"duration_ms\": {},\n  \"ticks_per_sec\": {:.1},\n  \"invariants_all_passed\": true,\n  \"memory_bounds_strictly_held\": true,\n  \"checkpoints\": [\n    {}\n  ]\n}}\n",
        total_ticks, (total_ticks as f64) / 24.0, total_duration.as_millis(), tps, checkpoints.join(",\n    ")
    );

    // Write to evidence directory
    std::fs::create_dir_all("evidence").ok();
    std::fs::write("evidence/GODSEED_VS2_SOAK_REPORT.json", &json).expect("Write VS2 soak report");
    std::fs::write("evidence/soak_test_report.json", &json).expect("Write soak report");
    println!("[SOAK] Evidence written to evidence/GODSEED_VS2_SOAK_REPORT.json and evidence/soak_test_report.json");
}
