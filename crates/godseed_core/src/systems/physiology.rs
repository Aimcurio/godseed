/// Physiology System — daily satiety/health decay for NPCs and player

use bevy_ecs::prelude::*;

use crate::components::{CitizenMeta, Demographics, PhysicalNeeds};
use crate::events::{DeathCause, SimEvent};
use crate::resources::EventRing;
use crate::types::SimClock;

/// Each tick (1 hour), satiety decays slightly.
/// At 24 ticks/day: losing ~4 satiety/day means you need to eat to maintain.
pub fn physiology_system(
    clock: Res<SimClock>,
    mut event_ring: ResMut<EventRing>,
    mut query: Query<(&mut CitizenMeta, &mut Demographics, &mut PhysicalNeeds)>,
) {
    for (mut meta, mut demo, mut needs) in query.iter_mut() {
        if !meta.alive { continue; }

        // Satiety decays at ~0.17 per tick (≈4/day)
        if clock.tick % 6 == 0 && needs.satiety > 0 {
            needs.satiety = needs.satiety.saturating_sub(1);
        }

        // Rest decays slightly each tick during daytime
        if clock.tick % 3 == 0 && needs.rest > 0 && clock.hour() >= 6 && clock.hour() <= 22 {
            needs.rest = needs.rest.saturating_sub(1);
        }


        // Health collapses when starving
        if needs.satiety == 0 {
            if demo.health > 0 {
                demo.health = demo.health.saturating_sub(2);
            }
            if demo.health == 0 {
                meta.alive = false;
                event_ring.emit(SimEvent::Death {
                    citizen: meta.id,
                    cause: DeathCause::Starvation,
                    tick: clock.tick,
                });
            }
        }

        // Slow health recovery when well-fed
        if needs.satiety > 80 && demo.health < 100 {
            demo.health = (demo.health + 1).min(100);
        }
    }
}

