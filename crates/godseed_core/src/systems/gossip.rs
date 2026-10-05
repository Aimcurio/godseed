/// Gossip System — weekly propagation of observations, episodic narratives, and asymmetric knowledge between NPCs
/// (AC-13, AC-203, AC-204, Architecture Final Sec 8)
use bevy_ecs::prelude::*;

use crate::components::PlayerMarker;
use crate::components::{
    CitizenMeta, EpisodicMemory, EpistemicState, NpcSchedule, RelationalLedger, SettlementRef,
};
use crate::events::SimEvent;
use crate::resources::EventRing;
use crate::types::{CitizenId, EpisodicRecord, LocationId, MemoryTag, NpcActivity, SimClock};

/// Weekly: NPCs share information about the player, episodic stories, and factual knowledge with others at the same location.
/// Gossip is one-hop only (no transitive chaining within one tick).
pub fn gossip_system(
    clock: Res<SimClock>,
    mut event_ring: ResMut<EventRing>,
    mut query: Query<
        (
            &CitizenMeta,
            &mut RelationalLedger,
            &mut EpisodicMemory,
            &mut EpistemicState,
            &SettlementRef,
            &NpcSchedule,
        ),
        Without<PlayerMarker>,
    >,
) {
    // Collect narrative stories, disposition, and knowledge info to share
    let mut narrative_snapshots: Vec<(CitizenId, String, LocationId, EpisodicRecord)> = Vec::new();
    let mut disposition_snapshots: Vec<(CitizenId, LocationId, i16, bool)> = Vec::new();
    let mut knowledge_snapshots: Vec<(CitizenId, LocationId, Vec<u16>)> = Vec::new();

    // First pass: gather current stories, dispositions, and knowledge from NPCs who are socializing
    for (meta, ledger, episodic, epistemic, settlement_ref, schedule) in query.iter() {
        if !meta.alive {
            continue;
        }
        let hour = clock.hour();
        let (activity, _) = schedule.activity_at_hour(hour);
        if activity == NpcActivity::Socializing {
            // Find high-impact episodic records regarding player to share as narrative gossip (AC-203)
            for record in episodic.all_records() {
                if record.actor == CitizenId::PLAYER || record.target == Some(CitizenId::PLAYER) {
                    // Only share firsthand experiences, not secondhand gossip (enforcing one-hop boundary)
                    if !matches!(record.tag, MemoryTag::HeardGossipAbout(_)) {
                        narrative_snapshots.push((
                            meta.id,
                            meta.name.clone(),
                            settlement_ref.current_location,
                            record.clone(),
                        ));
                    }
                }
            }

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
    for (meta, mut ledger, mut episodic, mut epistemic, settlement_ref, _schedule) in
        query.iter_mut()
    {
        if !meta.alive {
            continue;
        }

        // 1. Narrative Episodic Gossip (AC-203)
        for (speaker_id, _speaker_name, speaker_loc, story) in &narrative_snapshots {
            if *speaker_id == meta.id {
                continue;
            }
            if *speaker_loc != settlement_ref.current_location {
                continue;
            }

            // Check if listener has already heard gossip from this speaker
            let already_heard =
                episodic.has_record_with_tag(MemoryTag::HeardGossipAbout(*speaker_id));
            if already_heard {
                continue;
            }

            // Attenuated narrative impact
            let att_sentiment = (story.delta_sentiment / 2).clamp(-30, 30);
            let att_trust = (story.delta_trust / 2).clamp(-30, 30);

            let gossip_record = EpisodicRecord {
                id: clock.tick * 1000 + speaker_id.0,
                tick: clock.tick,
                actor: *speaker_id,
                target: Some(CitizenId::PLAYER),
                tag: MemoryTag::HeardGossipAbout(*speaker_id),
                delta_sentiment: att_sentiment,
                delta_trust: att_trust,
                delta_obligation: 0,
                is_permanent: true,
                narrative_token: story.narrative_token,
                causal: story.causal,
            };
            episodic.add_record(gossip_record);

            // Shift listener relational bond
            let delta_s = (att_sentiment / 2).clamp(-15, 15);
            let delta_t = (att_trust / 2).clamp(-15, 15);
            ledger.adjust(CitizenId::PLAYER, delta_s, delta_t, 0);

            event_ring.emit(SimEvent::GossipEvent {
                speaker: *speaker_id,
                listener: meta.id,
                about: CitizenId::PLAYER,
                tick: clock.tick,
            });
        }

        // 2. Disposition gossip (for general stance when no specific narrative story was shared)
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
            // Skip if listener already received a specific narrative episode from this speaker
            if episodic.has_record_with_tag(MemoryTag::HeardGossipAbout(*speaker_id)) {
                continue;
            }

            // Receive attenuated gossip
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

        // 3. Asymmetric Corroborating Epistemic Gossip (AC-204)
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
