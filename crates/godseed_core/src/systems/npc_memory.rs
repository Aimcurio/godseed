/// NPC Memory System — consolidate events into NPC memory structures
use bevy_ecs::prelude::*;

use crate::components::PlayerMarker;
use crate::components::{CitizenMeta, Disposition, NpcMemory};
use crate::events::SimEvent;
use crate::resources::EventRing;
use crate::types::{CitizenId, MemoryEvent, MemoryEventType, SimClock};

/// Each tick, scan recent events and update NPC memories / dispositions.
/// This is O(events × npcs) but with capped event ring and small NPC count it's negligible.
pub fn npc_memory_system(
    _clock: Res<SimClock>,
    event_ring: Res<EventRing>,
    mut query: Query<(&CitizenMeta, &mut NpcMemory, &mut Disposition), Without<PlayerMarker>>,
) {
    // Only process events emitted in the last tick
    let recent_events: Vec<&SimEvent> = event_ring.recent(50).collect();

    for (meta, mut memory, mut disposition) in query.iter_mut() {
        if !meta.alive {
            continue;
        }

        for event in &recent_events {
            match event {
                SimEvent::RelationshipEvent {
                    actor,
                    target,
                    delta,
                    tick,
                } => {
                    // If this NPC was the target (player did something to them)
                    if *target == meta.id && *actor == CitizenId::PLAYER {
                        let event_type = if *delta > 0 {
                            MemoryEventType::PlayerHelped
                        } else {
                            MemoryEventType::PlayerHarmed
                        };
                        let impact = (*delta).clamp(-5, 5) as i8;
                        memory.add_event(MemoryEvent {
                            tick: *tick,
                            event_type,
                            subject: CitizenId::PLAYER,
                            impact,
                            description: format!("Player interaction at tick {}", tick),
                        });
                        // Update disposition
                        disposition.toward_player =
                            (disposition.toward_player + *delta).clamp(-100, 100);
                    }
                }
                SimEvent::PlayerAction {
                    tick,
                    action_name,
                    success,
                } => {
                    // NPCs at the player's location "witness" player actions
                    // (simplified: all NPCs get a weak memory of significant player actions)
                    if *success && action_name.starts_with("Work") {
                        // Small positive toward hardworking player
                        memory.add_event(MemoryEvent {
                            tick: *tick,
                            event_type: MemoryEventType::PlayerWorked,
                            subject: CitizenId::PLAYER,
                            impact: 1,
                            description: format!("Saw player working ({})", action_name),
                        });
                        disposition.toward_player =
                            (disposition.toward_player + 1).clamp(-100, 100);
                    }
                }
                _ => {}
            }
        }
    }
}
