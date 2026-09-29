/// Godseed Systems — Pending Consequence Progression & Maturation (AC-206, AC-207)

use bevy_ecs::prelude::*;

use crate::components::{CitizenMeta, NpcSchedule, OccupationProfile, PlayerMarker};
use crate::events::SimEvent;
use crate::resources::{EventRing, EpistemicReturnDigest, PendingConsequenceRegistry, ReturnDigestLog};
use crate::types::{
    ConsequenceStage, ConsequenceType, OccupationType, SimClock, TriggerCondition,
};

/// Evaluates pending consequences and advances them through their causal lifecycle
pub fn pending_consequence_progression_system(
    clock: Res<SimClock>,
    mut consequences: ResMut<PendingConsequenceRegistry>,
    mut digest_log: ResMut<ReturnDigestLog>,
    mut event_ring: ResMut<EventRing>,
    mut npc_query: Query<
        (&CitizenMeta, &mut NpcSchedule, &mut OccupationProfile),
        Without<PlayerMarker>,
    >,
) {
    let current_tick = clock.tick;

    for consequence in consequences.consequences.iter_mut() {
        if consequence.stage != ConsequenceStage::Active && consequence.stage != ConsequenceStage::Escalated {
            continue;
        }

        let is_triggered = match &consequence.trigger {
            TriggerCondition::TimeElapsed { duration_ticks } => {
                current_tick >= consequence.created_tick + duration_ticks
            }
            TriggerCondition::Compound(conditions) => {
                conditions.iter().all(|c| match c {
                    TriggerCondition::TimeElapsed { duration_ticks } => {
                        current_tick >= consequence.created_tick + duration_ticks
                    }
                    _ => false,
                })
            }
        };

        if is_triggered {
            consequence.stage = ConsequenceStage::Matured;

            match consequence.consequence_type {
                ConsequenceType::FraternalLaborStrain { elder, junior, target_workplace } => {
                    // 1. Mutate junior's schedule and occupation to new workplace
                    for (meta, mut schedule, mut occ) in npc_query.iter_mut() {
                        if meta.id == junior {
                            schedule.work_location = target_workplace;
                            for slot in schedule.slots.iter_mut() {
                                if slot.activity == crate::types::NpcActivity::Working {
                                    slot.location = target_workplace;
                                }
                            }
                            occ.occupation = OccupationType::Artisan;
                        }
                    }

                    // 2. Emit consequence matured simulation event
                    event_ring.emit(SimEvent::ConsequenceMatured {
                        consequence_id: consequence.id,
                        causal_root: consequence.causal_root,
                        tick: current_tick,
                    });

                    // 3. Register return digest salutation for when player speaks with elder
                    digest_log.push(EpistemicReturnDigest {
                        tick: current_tick,
                        speaker: elder,
                        causal_root: consequence.causal_root,
                        message: "Tomas Birch speaks with quiet regret: 'I haven't forgotten how you stood with me at the woodlot. But Runn took it hard... he felt he wasn't needed. He's taken an apprenticeship with Wren at the forge. I work the timber alone now.'".to_string(),
                    });
                }
            }
        }
    }
}
