/// Gossip System — weekly propagation of observations between NPCs

use bevy_ecs::prelude::*;

use crate::components::{CitizenMeta, Disposition, NpcMemory, NpcSchedule, SettlementRef};
use crate::components::PlayerMarker;
use crate::events::SimEvent;
use crate::resources::EventRing;
use crate::types::{CitizenId, LocationId, MemoryEvent, MemoryEventType, NpcActivity, SimClock};

/// Weekly: NPCs share information about the player with others at the same location.
/// Gossip is one-hop only (no transitive chaining within one tick).
pub fn gossip_system(
    clock: Res<SimClock>,
    mut event_ring: ResMut<EventRing>,
    mut query: Query<
        (&CitizenMeta, &mut NpcMemory, &mut Disposition, &SettlementRef, &NpcSchedule),
        Without<PlayerMarker>,
    >,
) {
    // Collect disposition info to share
    let mut disposition_snapshots: Vec<(CitizenId, LocationId, i16, bool)> = Vec::new();

    // First pass: gather current dispositions from NPCs who are socializing
    for (meta, _memory, disposition, settlement_ref, schedule) in query.iter() {
        if !meta.alive { continue; }
        let hour = clock.hour();
        let (activity, _) = schedule.activity_at_hour(hour);
        if activity == NpcActivity::Socializing {
            let has_strong_opinion = disposition.toward_player.abs() > 20;
            disposition_snapshots.push((
                meta.id,
                settlement_ref.current_location,
                disposition.toward_player,
                has_strong_opinion,
            ));
        }
    }

    // Second pass: share with NPCs at the same location
    for (meta, mut memory, mut disposition, settlement_ref, _schedule) in query.iter_mut() {
        if !meta.alive { continue; }

        for (speaker_id, speaker_loc, speaker_disp, strong) in &disposition_snapshots {
            if *speaker_id == meta.id { continue; }
            if *speaker_loc != settlement_ref.current_location { continue; }
            if !strong { continue; }

            // Receive attenuated gossip (half impact)
            let gossip_impact = ((*speaker_disp as f32) * 0.15) as i16;
            if gossip_impact.abs() < 2 { continue; }

            memory.add_event(MemoryEvent {
                tick: clock.tick,
                event_type: MemoryEventType::HeardGossip { from: *speaker_id },
                subject: CitizenId::PLAYER,
                impact: gossip_impact.clamp(-3, 3) as i8,
                description: format!("Heard about player from NPC#{}", speaker_id.0),
            });

            disposition.toward_player = (disposition.toward_player + gossip_impact / 3).clamp(-100, 100);

            event_ring.emit(SimEvent::GossipEvent {
                speaker: *speaker_id,
                listener: meta.id,
                about: CitizenId::PLAYER,
                tick: clock.tick,
            });
        }
    }
}
