/// Gossip System — weekly propagation of observations and asymmetric knowledge between NPCs
/// (AC-13, AC-204, Architecture Final Sec 8)
use bevy_ecs::prelude::*;

use crate::components::PlayerMarker;
use crate::components::{
    CitizenMeta, EpistemicState, NpcSchedule, RelationalLedger, SettlementRef,
};
use crate::events::SimEvent;
use crate::resources::EventRing;
use crate::types::{CitizenId, LocationId, NpcActivity, SimClock};

/// Weekly: NPCs share information about the player and factual knowledge with others at the same location.
/// Gossip is one-hop only (no transitive chaining within one tick).
pub fn gossip_system(
    clock: Res<SimClock>,
    mut event_ring: ResMut<EventRing>,
    mut query: Query<
        (
            &CitizenMeta,
            &mut RelationalLedger,
            &mut EpistemicState,
            &SettlementRef,
            &NpcSchedule,
        ),
        Without<PlayerMarker>,
    >,
) {
    // Collect disposition and knowledge info to share
    let mut disposition_snapshots: Vec<(CitizenId, LocationId, i16, bool)> = Vec::new();
    let mut knowledge_snapshots: Vec<(CitizenId, LocationId, Vec<u16>)> = Vec::new();

    // First pass: gather current dispositions and knowledge from NPCs who are socializing
    for (meta, ledger, epistemic, settlement_ref, schedule) in query.iter() {
        if !meta.alive {
            continue;
        }
        let hour = clock.hour();
        let (activity, _) = schedule.activity_at_hour(hour);
        if activity == NpcActivity::Socializing {
            let bond = ledger.get_bond(CitizenId::PLAYER);
            let score = ((bond.sentiment as i16 + bond.trust as i16) / 2).clamp(-100, 100);
            let has_strong_opinion = score.abs() > 20;
            disposition_snapshots.push((
                meta.id,
                settlement_ref.current_location,
                score,
                has_strong_opinion,
            ));

            let known_keys: Vec<u16> = epistemic.known.keys().copied().collect();
            if !known_keys.is_empty() {
                knowledge_snapshots.push((meta.id, settlement_ref.current_location, known_keys));
            }
        }
    }

    // Second pass: share with NPCs at the same location
    for (meta, mut ledger, mut epistemic, settlement_ref, _schedule) in query.iter_mut() {
        if !meta.alive {
            continue;
        }

        // 1. Disposition gossip
        for (speaker_id, speaker_loc, speaker_disp, strong) in &disposition_snapshots {
            if *speaker_id == meta.id {
                continue;
            }
            if *speaker_loc != settlement_ref.current_location {
                continue;
            }
            if !strong {
                continue;
            }

            // Receive attenuated gossip (half impact)
            let gossip_impact = ((*speaker_disp as f32) * 0.15) as i16;
            if gossip_impact.abs() < 2 {
                continue;
            }

            let delta = (gossip_impact / 3).clamp(-3, 3) as i8;
            ledger.adjust(CitizenId::PLAYER, delta, delta, 0);

            event_ring.emit(SimEvent::GossipEvent {
                speaker: *speaker_id,
                listener: meta.id,
                about: CitizenId::PLAYER,
                tick: clock.tick,
            });
        }

        // 2. Asymmetric Corroborating Epistemic Gossip (AC-204)
        for (speaker_id, speaker_loc, speaker_nodes) in &knowledge_snapshots {
            if *speaker_id == meta.id {
                continue;
            }
            if *speaker_loc != settlement_ref.current_location {
                continue;
            }

            for &k_id in speaker_nodes {
                // If novel or corroboration count < 3, epistemic.learn returns true.
                // If already at max saturation (>= 3), returns false (no-op suppression).
                if epistemic.learn(k_id, clock.tick) {
                    let corroboration = epistemic.get_corroboration(k_id);
                    event_ring.emit(SimEvent::KnowledgeShared {
                        speaker: *speaker_id,
                        listener: meta.id,
                        knowledge_id: k_id,
                        corroboration,
                        tick: clock.tick,
                    });
                }
            }
        }
    }
}
