/// Transformation System — monthly check for Inscription path progress

use bevy_ecs::prelude::*;

use crate::components::{
    CapabilitySet, CitizenMeta, Disposition, KnowledgeInventory, PlayerMarker, TransformationState,
};
use crate::content::{caps, knowledge, milestones};
use crate::events::SimEvent;
use crate::resources::{EventRing, RelationshipLedger};
use crate::types::{CitizenId, SimClock, TransformationPath};

/// Monthly: check if player meets conditions to advance the Inscription path.
pub fn transformation_check_system(
    clock: Res<SimClock>,
    relationships: Res<RelationshipLedger>,
    mut event_ring: ResMut<EventRing>,
    mut player_query: Query<
        (
            &CitizenMeta,
            &mut TransformationState,
            &mut CapabilitySet,
            &KnowledgeInventory,
        ),
        With<PlayerMarker>,
    >,
    _npc_dispositions: Query<(&CitizenMeta, &mut Disposition), Without<PlayerMarker>>,
) {
    for (meta, mut transform, capabilities, knowledge) in player_query.iter_mut() {
        if !meta.alive { continue; }

        match transform.path {
            TransformationPath::None => {
                let knows_archive = knowledge.knows(knowledge::ANCIENT_ARCHIVE);
                let has_inscription = capabilities.has(caps::INSCRIPTION);
                let elder_relationship = relationships.get(CitizenId::PLAYER, CitizenId(5));

                if has_inscription || (knows_archive && elder_relationship > 20) {
                    transform.path = TransformationPath::Inscription;
                    if has_inscription {
                        transform.stage = 1;
                        transform.progress = 10;
                        if !transform.milestones.contains(&milestones::INSCRIPTION_LEARNED) {
                            transform.milestones.push(milestones::INSCRIPTION_LEARNED);
                        }
                    } else {
                        transform.stage = 0;
                        transform.progress = 10;
                    }

                    event_ring.emit(SimEvent::TransformationEvent { stage: transform.stage, tick: clock.tick });
                }
            }


            TransformationPath::Inscription => {
                match transform.stage {
                    0 => {
                        // Stage 0 → 1 (Scholar): Must have INSCRIPTION capability
                        if capabilities.has(caps::INSCRIPTION) {
                            if !transform.milestones.contains(&milestones::INSCRIPTION_LEARNED) {
                                transform.milestones.push(milestones::INSCRIPTION_LEARNED);
                            }
                            transform.stage = 1;
                            transform.progress = 0;

                            event_ring.emit(SimEvent::TransformationEvent { stage: 1, tick: clock.tick });
                            event_ring.emit(SimEvent::CapabilityAcquired {
                                citizen: CitizenId::PLAYER,
                                capability_id: caps::INSCRIPTION.0,
                                tick: clock.tick,
                            });
                        }
                    }
                    1 => {
                        // Stage 1 (Scholar): Track inscriptions toward recognition
                        if transform.inscriptions_completed >= 5
                            && !transform.milestones.contains(&milestones::FIVE_INSCRIPTIONS)
                        {
                            transform.milestones.push(milestones::FIVE_INSCRIPTIONS);
                            transform.progress = (transform.progress + 30).min(100);

                            event_ring.emit(SimEvent::TransformationEvent { stage: 1, tick: clock.tick });
                        }

                        // Scholar recognition: Elder Voss relationship > 60 AND 5 inscriptions
                        let elder_rel = relationships.get(CitizenId::PLAYER, CitizenId(5));
                        if transform.milestones.contains(&milestones::FIVE_INSCRIPTIONS)
                            && elder_rel > 60
                            && !transform.milestones.contains(&milestones::SCHOLAR_RECOGNIZED)
                        {
                            transform.milestones.push(milestones::SCHOLAR_RECOGNIZED);
                            transform.progress = 100;
                        }
                    }
                    2 | 3 => {
                        // Seamed — architecture exists but not implemented in VS1
                    }
                    _ => {}
                }
            }
        }
    }
}
