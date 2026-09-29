/// Godseed — State hash for replay verification (FNV-1a, inherited from CIVITAS-1M)

use bevy_ecs::prelude::*;
use fnv::FnvHasher;
use std::hash::{Hash, Hasher};

use crate::components::{CitizenMeta, Demographics, PersonalFinances, PhysicalNeeds};
use crate::types::SimClock;

/// Compute a deterministic 64-bit hash of NPC-only state (excluding player input and telemetry).
/// This hash is used for determinism verification and save/load regression tests.
pub fn compute_authoritative_state_hash(world: &mut World) -> u64 {
    let mut hasher = FnvHasher::default();

    // Hash sim clock
    let tick = world.resource::<SimClock>().tick;
    tick.hash(&mut hasher);

    // Hash all citizen states (sorted by CitizenId for determinism)
    let mut citizen_states: Vec<(u64, u8, u64, i64)> = Vec::new();

    let mut query = world.query::<(&CitizenMeta, &Demographics, &PhysicalNeeds, &PersonalFinances)>();
    for (meta, demo, _needs, finances) in query.iter(world) {
        citizen_states.push((
            meta.id.0,
            if meta.alive { 1 } else { 0 },
            demo.age_ticks as u64 + demo.age_years as u64 * 10_000,
            (finances.coins * 100.0) as i64, // Fixed-point to avoid float hash instability
        ));
    }

    // Sort for determinism
    citizen_states.sort_by_key(|(id, _, _, _)| *id);
    for state in &citizen_states {
        state.hash(&mut hasher);
    }

    hasher.finish()
}
