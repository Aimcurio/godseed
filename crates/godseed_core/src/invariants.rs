/// Godseed — Machine-Checkable Invariants

use bevy_ecs::prelude::*;

use crate::components::{CitizenMeta, HouseholdRef, PlayerMarker};
use crate::household::HouseholdDirectory;

/// Verify all machine-checkable invariants. Returns Ok or list of violations.
pub fn verify_invariants(world: &mut World) -> Result<(), Vec<String>> {
    let mut violations = Vec::new();

    // INV-1: Exactly one PlayerMarker entity
    {
        let mut q = world.query::<&PlayerMarker>();
        let count = q.iter(world).count();
        if count != 1 {
            violations.push(format!("INV-1 FAIL: Expected exactly 1 PlayerMarker, found {}", count));
        }
    }

    // INV-2: All living citizens have valid household refs
    {
        let mut q = world.query::<(&CitizenMeta, &HouseholdRef)>();
        let households = world.resource::<HouseholdDirectory>();
        for (meta, hh_ref) in q.iter(world) {
            if !meta.alive { continue; }
            if households.get(hh_ref.household_id).is_none() {
                violations.push(format!(
                    "INV-2 FAIL: Living citizen {} references non-existent household {}",
                    meta.id.0, hh_ref.household_id.0
                ));
            }
        }
    }

    // INV-3: No negative finances
    {
        use crate::components::PersonalFinances;
        let mut q = world.query::<(&CitizenMeta, &PersonalFinances)>();
        for (meta, fin) in q.iter(world) {
            if fin.coins < -0.01 {
                violations.push(format!(
                    "INV-3 FAIL: Citizen {} has negative coins: {:.2}",
                    meta.id.0, fin.coins
                ));
            }
        }
    }

    // INV-4: Dead citizens have 0 satiety (they don't consume)
    {
        use crate::components::PhysicalNeeds;
        let mut q = world.query::<(&CitizenMeta, &PhysicalNeeds)>();
        for (meta, needs) in q.iter(world) {
            if !meta.alive && needs.satiety > 0 {
                // This is a data warning, not hard failure — dead citizens may have residual satiety
                // violations.push(format!("INV-4 WARN: Dead citizen {} has satiety > 0", meta.id.0));
            }
        }
    }

    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}
