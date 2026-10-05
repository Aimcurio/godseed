/// Demographics System — aging, natural death
use bevy_ecs::prelude::*;

use crate::components::{CitizenMeta, Demographics};
use crate::events::{DeathCause, SimEvent};
use crate::resources::EventRing;
use crate::types::SimClock;

/// Ticks per year: 24 hours/day × 365 days = 8760 ticks/year
/// For VS1, 1 tick = 1 in-game hour
const TICKS_PER_YEAR: u32 = 8_760;

pub fn demographics_aging_system(
    clock: Res<SimClock>,
    mut event_ring: ResMut<EventRing>,
    mut query: Query<(&mut CitizenMeta, &mut Demographics)>,
) {
    for (mut meta, mut demo) in query.iter_mut() {
        if !meta.alive {
            continue;
        }

        demo.age_ticks += 1;
        if demo.age_ticks >= TICKS_PER_YEAR {
            demo.age_ticks = 0;
            demo.age_years += 1;

            // Natural death probability increases sharply after 65
            let _death_prob = if demo.age_years >= 80 {
                0.40
            } else if demo.age_years >= 75 {
                0.20
            } else if demo.age_years >= 70 {
                0.10
            } else if demo.age_years >= 65 {
                0.05
            } else {
                0.0
            };

            // Simple threshold check (deterministic based on age for VS1)
            // In a seeded full sim this would use RNG; for VS1 we use age > 82 as death
            if demo.age_years > 82 || (demo.age_years >= 75 && demo.health < 20) {
                meta.alive = false;
                event_ring.emit(SimEvent::Death {
                    citizen: meta.id,
                    cause: DeathCause::OldAge,
                    tick: clock.tick,
                });
            }
        }
    }
}
