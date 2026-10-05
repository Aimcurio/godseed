/// Player Action System — drain PlayerInputBuffer and resolve actions
///
/// This is the bridge between UI input and simulation state.
/// Actions are validated, executed, and produce ActionResults.
use bevy_ecs::prelude::*;

use crate::components::{
    CapabilitySet, CitizenMeta, EpisodicMemory, EpistemicState, Inventory, KnowledgeInventory,
    NpcSchedule, NpcSocialProfile, OccupationProfile, PersonalFinances, PhysicalNeeds,
    PlayerInputBuffer, PlayerMarker, RelationalLedger, SettlementRef, TransformationState,
};
use crate::content::{caps, knowledge, ContentDefinitions};
use crate::events::{SimEvent, TelemetryEvent};
use crate::resources::{
    DocumentRegistry, EventRing, NextCausalId, PendingConsequenceRegistry, ReputationRegistry,
    ReturnDigestLog, TelemetryLog,
};
use crate::settlement::SettlementDirectory;
use crate::types::{
    ActionResult, BehavioralMode, CapabilityId, CapabilityLevel, CausalPointer, CitizenId,
    ConsequenceStage, ConsequenceType, DocumentType, EpisodicRecord, Exchange, InscribedDocument,
    LocationId, MemoryTag, OccupationType, PlayerAction, RelationalBond, ResourceType, SideEffect,
    SimClock, TalkTopic, TriggerCondition,
};
use crate::world::WorldMap;

/// Main action processing system
pub fn player_action_system(
    clock: Res<SimClock>,
    world_map: Res<WorldMap>,
    content: Res<ContentDefinitions>,
    mut settlements: ResMut<SettlementDirectory>,
    mut reputation: ResMut<ReputationRegistry>,
    mut event_ring: ResMut<EventRing>,
    mut telemetry: ResMut<TelemetryLog>,
    mut next_causal: ResMut<NextCausalId>,
    mut consequences: ResMut<PendingConsequenceRegistry>,
    mut return_digests: ResMut<ReturnDigestLog>,
    mut documents: ResMut<DocumentRegistry>,
    mut player_query: Query<
        (
            &mut CitizenMeta,
            &mut PlayerInputBuffer,
            &mut SettlementRef,
            &mut PersonalFinances,
            &mut PhysicalNeeds,
            &mut Inventory,
            &mut CapabilitySet,
            &mut KnowledgeInventory,
            &mut TransformationState,
            &mut EpistemicState,
        ),
        With<PlayerMarker>,
    >,
    npc_query: Query<
        (
            &CitizenMeta,
            &NpcSocialProfile,
            &NpcSchedule,
            &SettlementRef,
            &OccupationProfile,
        ),
        Without<PlayerMarker>,
    >,
    mut episodic_query: Query<
        (
            &CitizenMeta,
            &mut EpisodicMemory,
            &mut RelationalLedger,
            &mut EpistemicState,
        ),
        Without<PlayerMarker>,
    >,
) {
    for (
        mut player_meta,
        mut input_buf,
        mut settlement_ref,
        mut finances,
        mut needs,
        mut inventory,
        mut capabilities,
        mut knowledge_inv,
        mut transform,
        mut player_epistemic,
    ) in player_query.iter_mut()
    {
        // Process one action per tick
        if let Some(action) = input_buf.pop_action() {
            let result = resolve_action(
                clock.tick,
                &action,
                &mut player_meta,
                &mut settlement_ref,
                &mut finances,
                &mut needs,
                &mut inventory,
                &mut capabilities,
                &mut knowledge_inv,
                &mut transform,
                &mut player_epistemic,
                &world_map,
                &content,
                &mut settlements,
                &mut reputation,
                &mut event_ring,
                &mut next_causal,
                &mut consequences,
                &mut return_digests,
                &mut documents,
                &npc_query,
                &mut episodic_query,
            );

            telemetry.emit(TelemetryEvent::player_action(
                clock.tick,
                &format!("{:?}", action).chars().take(40).collect::<String>(),
                &result.message.chars().take(60).collect::<String>(),
            ));

            input_buf.push_result(result);
        }
    }
}

/// Resolve a single player action → ActionResult
fn resolve_action(
    tick: u64,
    action: &PlayerAction,
    meta: &mut CitizenMeta,
    settlement_ref: &mut SettlementRef,
    finances: &mut PersonalFinances,
    needs: &mut PhysicalNeeds,
    inventory: &mut Inventory,
    capabilities: &mut CapabilitySet,
    knowledge_inv: &mut KnowledgeInventory,
    transform: &mut TransformationState,
    player_epistemic: &mut EpistemicState,
    world_map: &WorldMap,
    content: &ContentDefinitions,
    settlements: &mut SettlementDirectory,
    _reputation: &mut ReputationRegistry,
    event_ring: &mut EventRing,
    next_causal: &mut NextCausalId,
    consequences: &mut PendingConsequenceRegistry,
    return_digests: &mut ReturnDigestLog,
    documents: &mut DocumentRegistry,
    npc_query: &Query<
        (
            &CitizenMeta,
            &NpcSocialProfile,
            &NpcSchedule,
            &SettlementRef,
            &OccupationProfile,
        ),
        Without<PlayerMarker>,
    >,
    episodic_query: &mut Query<
        (
            &CitizenMeta,
            &mut EpisodicMemory,
            &mut RelationalLedger,
            &mut EpistemicState,
        ),
        Without<PlayerMarker>,
    >,
) -> ActionResult {
    if !meta.alive {
        return ActionResult {
            tick,
            success: false,
            message: "You are deceased. The dead cannot act in the realm of the living."
                .to_string(),
            side_effects: vec![],
        };
    }

    match action {
        PlayerAction::Move { to } => resolve_move(tick, settlement_ref, world_map, *to),
        PlayerAction::Look => resolve_look(tick, settlement_ref, world_map, npc_query),
        PlayerAction::Inspect { target } => {
            resolve_inspect(tick, *target, content, npc_query, episodic_query)
        }
        PlayerAction::Talk { npc, topic } => resolve_talk(
            tick,
            *npc,
            topic,
            meta,
            settlement_ref,
            knowledge_inv,
            player_epistemic,
            event_ring,
            return_digests,
            npc_query,
            content,
            transform,
            consequences,
            episodic_query,
        ),
        PlayerAction::HelpWithFelling { npc } => resolve_help_with_felling(
            tick,
            *npc,
            settlement_ref,
            event_ring,
            next_causal,
            consequences,
            npc_query,
            episodic_query,
        ),
        PlayerAction::Diagnose { location } => resolve_diagnose(
            tick,
            *location,
            capabilities,
            transform,
            settlement_ref,
            player_epistemic,
            event_ring,
        ),
        PlayerAction::DraftDocument { doc_type } => resolve_draft_document(
            tick,
            doc_type,
            capabilities,
            transform,
            player_epistemic,
            documents,
            event_ring,
        ),
        PlayerAction::ArbitrateDispute {
            document_id,
            consequence_id,
        } => resolve_arbitrate_dispute(
            tick,
            *document_id,
            *consequence_id,
            documents,
            consequences,
            episodic_query,
            next_causal,
            event_ring,
        ),
        PlayerAction::Offer { npc, exchange } => resolve_offer(
            tick,
            *npc,
            exchange,
            finances,
            inventory,
            capabilities,
            episodic_query,
            settlements,
            event_ring,
        ),
        PlayerAction::Buy { resource, quantity } => resolve_buy(
            tick,
            *resource,
            *quantity,
            finances,
            inventory,
            settlements,
            event_ring,
        ),
        PlayerAction::Sell { resource, quantity } => resolve_sell(
            tick,
            *resource,
            *quantity,
            finances,
            inventory,
            settlements,
            event_ring,
        ),
        PlayerAction::Work { occupation } => resolve_work(
            tick,
            *occupation,
            finances,
            needs,
            inventory,
            capabilities,
            settlement_ref,
            settlements,
            event_ring,
        ),
        PlayerAction::Practice { capability } => resolve_practice(
            tick,
            *capability,
            capabilities,
            content,
            event_ring,
            inventory,
        ),
        PlayerAction::LearnFrom { npc, capability } => resolve_learn_from(
            tick,
            *npc,
            *capability,
            capabilities,
            knowledge_inv,
            event_ring,
            npc_query,
            content,
            episodic_query,
        ),
        PlayerAction::Inscribe { observation } => resolve_inscribe(
            tick,
            observation,
            transform,
            capabilities,
            knowledge_inv,
            event_ring,
        ),
        PlayerAction::StudyArchive => resolve_study_archive(
            tick,
            transform,
            capabilities,
            knowledge_inv,
            settlement_ref,
            event_ring,
            content,
        ),
        PlayerAction::Wait { ticks } => ActionResult {
            tick,
            success: true,
            message: format!("You wait for {} hours. Time passes.", ticks),
            side_effects: vec![],
        },
        PlayerAction::Sleep => resolve_sleep(tick, needs, settlement_ref, finances, settlements),
        PlayerAction::Save { path: _ } => ActionResult {
            tick,
            success: true,
            message: "Save requested (handled by simulation layer).".to_string(),
            side_effects: vec![],
        },
        PlayerAction::InspectObject { object: _ } => ActionResult {
            tick,
            success: false,
            message: format!("You examine the object but see nothing remarkable."),
            side_effects: vec![],
        },
        PlayerAction::PickUp { object: _ } => ActionResult {
            tick,
            success: false,
            message: "You can't pick that up.".to_string(),
            side_effects: vec![],
        },
        PlayerAction::Drop { object: _ } => ActionResult {
            tick,
            success: false,
            message: "You don't have that.".to_string(),
            side_effects: vec![],
        },
    }
}

fn canonical_relationship_score(
    npc_id: CitizenId,
    episodic_query: &mut Query<
        (
            &CitizenMeta,
            &mut EpisodicMemory,
            &mut RelationalLedger,
            &mut EpistemicState,
        ),
        Without<PlayerMarker>,
    >,
) -> i16 {
    episodic_query
        .iter_mut()
        .find(|(meta, _, _, _)| meta.id == npc_id)
        .map(|(_, _, ledger, _)| {
            let bond = ledger.get_bond(CitizenId::PLAYER);
            ((bond.sentiment as i16 + bond.trust as i16) / 2).clamp(-100, 100)
        })
        .unwrap_or(0)
}

fn adjust_canonical_relationship(
    npc_id: CitizenId,
    delta_sentiment: i8,
    delta_trust: i8,
    delta_obligation: i16,
    episodic_query: &mut Query<
        (
            &CitizenMeta,
            &mut EpisodicMemory,
            &mut RelationalLedger,
            &mut EpistemicState,
        ),
        Without<PlayerMarker>,
    >,
) {
    if let Some((_, _, mut ledger, _)) = episodic_query
        .iter_mut()
        .find(|(meta, _, _, _)| meta.id == npc_id)
    {
        ledger.adjust(
            CitizenId::PLAYER,
            delta_sentiment,
            delta_trust,
            delta_obligation,
        );
    }
}

// ── Action Resolvers ──────────────────────────────────────────────────────────

fn resolve_move(
    tick: u64,
    settlement_ref: &mut SettlementRef,
    world_map: &WorldMap,
    to: LocationId,
) -> ActionResult {
    let from = settlement_ref.current_location;

    // Check adjacency
    if !world_map.are_adjacent(from, to) && from != to {
        let from_name = world_map
            .get_location(from)
            .map(|l| l.name.as_str())
            .unwrap_or("here");
        let to_name = world_map
            .get_location(to)
            .map(|l| l.name.as_str())
            .unwrap_or("there");
        return ActionResult {
            tick,
            success: false,
            message: format!(
                "You can't reach {} directly from {}. Find a connected path.",
                to_name, from_name
            ),
            side_effects: vec![],
        };
    }

    let to_name = world_map
        .get_location(to)
        .map(|l| l.name.clone())
        .unwrap_or_else(|| format!("Location {}", to.0));
    let desc = world_map
        .get_location(to)
        .map(|l| l.description.clone())
        .unwrap_or_default();

    settlement_ref.current_location = to;

    ActionResult {
        tick,
        success: true,
        message: format!("You move to {}.\n{}", to_name, desc),
        side_effects: vec![SideEffect::PlayerMoved { to }],
    }
}

fn resolve_look(
    tick: u64,
    settlement_ref: &SettlementRef,
    world_map: &WorldMap,
    npc_query: &Query<
        (
            &CitizenMeta,
            &NpcSocialProfile,
            &NpcSchedule,
            &SettlementRef,
            &OccupationProfile,
        ),
        Without<PlayerMarker>,
    >,
) -> ActionResult {
    let loc = settlement_ref.current_location;
    let location = world_map.get_location(loc);

    let loc_name = location
        .map(|l| l.name.as_str())
        .unwrap_or("Unknown location");
    let loc_desc = location.map(|l| l.description.as_str()).unwrap_or("");

    // Find NPCs at this location
    let npcs_here: Vec<String> = npc_query
        .iter()
        .filter(|(meta, _, _, sref, _)| meta.alive && sref.current_location == loc)
        .map(|(meta, _, schedule, _, _)| {
            format!(
                "  • {} ({} — {})",
                meta.name,
                schedule.current_activity.display(),
                loc_name
            )
        })
        .collect();

    // Adjacent locations
    let adjacent = location
        .map(|l| {
            l.adjacent
                .iter()
                .filter_map(|id| {
                    world_map
                        .get_location(*id)
                        .map(|nl| nl.name.as_str().to_string())
                })
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_default();

    let mut msg = format!("═══ {} ═══\n{}\n", loc_name, loc_desc);
    if !npcs_here.is_empty() {
        msg.push_str("\nPeople here:\n");
        msg.push_str(&npcs_here.join("\n"));
    } else {
        msg.push_str("\nNo one else is here.");
    }
    if !adjacent.is_empty() {
        msg.push_str(&format!("\n\nConnected to: {}", adjacent));
    }

    ActionResult {
        tick,
        success: true,
        message: msg,
        side_effects: vec![],
    }
}

fn resolve_inspect(
    tick: u64,
    target: CitizenId,
    content: &ContentDefinitions,
    npc_query: &Query<
        (
            &CitizenMeta,
            &NpcSocialProfile,
            &NpcSchedule,
            &SettlementRef,
            &OccupationProfile,
        ),
        Without<PlayerMarker>,
    >,
    episodic_query: &mut Query<
        (
            &CitizenMeta,
            &mut EpisodicMemory,
            &mut RelationalLedger,
            &mut EpistemicState,
        ),
        Without<PlayerMarker>,
    >,
) -> ActionResult {
    if let Some((meta, _profile, schedule, _sref, occ)) =
        npc_query.iter().find(|(m, _, _, _, _)| m.id == target)
    {
        let bond = episodic_query
            .iter()
            .find(|(m, _, _, _)| m.id == target)
            .map(|(_, _, l, _)| l.get_bond(CitizenId::PLAYER))
            .unwrap_or_default();

        let def = content
            .npc_definitions
            .iter()
            .find(|d| d.citizen_id == target.0);

        let description = def
            .map(|d| d.description.as_str())
            .unwrap_or("A person you don't know much about.");
        let occ_name = occ.occupation.display_name();

        let disposition_note = match bond.mode() {
            BehavioralMode::DevotedAlly => {
                "They look upon you with devoted warmth and deep respect."
            }
            BehavioralMode::GrudgingDebtor => {
                "They watch you with sour resentment, mindful of their obligations."
            }
            BehavioralMode::AffectionateRefusal => {
                "They look upon you with affectionate warmth, though a flicker of caution remains."
            }
            BehavioralMode::HardenedEnemy => "They glare at you with open hostility.",
            BehavioralMode::WaryConsultant => {
                let rel = canonical_relationship_score(target, episodic_query);
                if rel > 20 {
                    "They regard you with mild goodwill."
                } else if rel < -20 {
                    "They seem wary of you."
                } else {
                    "They regard you with neutral curiosity."
                }
            }
        };

        let activity = schedule.current_activity.display();

        ActionResult {
            tick,
            success: true,
            message: format!(
                "══ {} ══\nOccupation: {}\nCurrently: {}\n\n{}\n\n{}",
                meta.name, occ_name, activity, description, disposition_note
            ),
            side_effects: vec![],
        }
    } else {
        ActionResult {
            tick,
            success: false,
            message: "You don't see that person here.".to_string(),
            side_effects: vec![],
        }
    }
}

fn resolve_talk(
    tick: u64,
    npc_id: CitizenId,
    topic: &TalkTopic,
    _player_meta: &CitizenMeta,
    settlement_ref: &SettlementRef,
    knowledge_inv: &mut KnowledgeInventory,
    player_epistemic: &mut EpistemicState,
    event_ring: &mut EventRing,
    return_digests: &mut ReturnDigestLog,
    npc_query: &Query<
        (
            &CitizenMeta,
            &NpcSocialProfile,
            &NpcSchedule,
            &SettlementRef,
            &OccupationProfile,
        ),
        Without<PlayerMarker>,
    >,
    content: &ContentDefinitions,
    _transform: &TransformationState,
    consequences: &PendingConsequenceRegistry,
    episodic_query: &mut Query<
        (
            &CitizenMeta,
            &mut EpisodicMemory,
            &mut RelationalLedger,
            &mut EpistemicState,
        ),
        Without<PlayerMarker>,
    >,
) -> ActionResult {
    let npc_result = npc_query.iter().find(|(m, _, _, _, _)| m.id == npc_id);

    if npc_result.is_none() {
        return ActionResult {
            tick,
            success: false,
            message: "You don't see that person here.".to_string(),
            side_effects: vec![],
        };
    }

    let (npc_meta, _profile, _schedule, npc_sref, occ) = npc_result.unwrap();

    // Check if NPC is accessible (same location)
    if npc_sref.current_location != settlement_ref.current_location {
        return ActionResult {
            tick,
            success: false,
            message: format!("{} isn't here right now.", npc_meta.name),
            side_effects: vec![],
        };
    }

    let bond = episodic_query
        .iter()
        .find(|(m, _, _, _)| m.id == npc_id)
        .map(|(_, _, l, _)| l.get_bond(CitizenId::PLAYER))
        .unwrap_or_default();
    let rel = canonical_relationship_score(npc_id, episodic_query);
    let _def = content
        .npc_definitions
        .iter()
        .find(|d| d.citizen_id == npc_id.0);

    let (message, side_effects, rel_delta) = match topic {
        TalkTopic::Greeting => {
            // 1. Check for unconsumed epistemic return digest for this speaker (AC-208)
            if let Some(digest) = return_digests.pop_digest_for(npc_id) {
                (digest.message, vec![], 2i16)
            } else {
                // 2. Check for VS2 specific felling consequence dialogue
                let is_fraternal_matured = consequences.consequences.iter().any(|c| {
                    matches!(
                        c.consequence_type,
                        ConsequenceType::FraternalLaborStrain { .. }
                    ) && c.stage == ConsequenceStage::Matured
                });
                let has_helped_felling = episodic_query.iter().any(|(meta, mem, _, _)| {
                    meta.id == CitizenId(6) && mem.has_anchor_with_tag(MemoryTag::HelpedWithFelling)
                });
                let has_heard_tomas_gossip = episodic_query.iter().any(|(meta, mem, _, _)| {
                    meta.id == npc_id
                        && mem.has_record_with_tag(MemoryTag::HeardGossipAbout(CitizenId(6)))
                });

                let response = if npc_id == CitizenId(6) && has_helped_felling {
                    if is_fraternal_matured {
                        format!("{} smiles warmly at you, though his eyes look tired. \"I haven't forgotten how you stood with me felling the oak. But... Runn took it hard. He felt displaced, like he was no longer needed here. He packed his kit and apprenticed with Wren at the forge. I work alone now.\"", npc_meta.name)
                    } else {
                        format!("{} smiles warmly and clasps your shoulder. \"Good to see you, friend. My back still remembers the oak we brought down together.\"", npc_meta.name)
                    }
                } else if npc_id == CitizenId(12) && is_fraternal_matured {
                    format!("{} wipes iron grime from his leather apron, looking at you with proud defiance. \"Wren took me on at the forge. Tomas didn't need two sets of hands at the woodlot anymore—not after you showed him how quick the felling could be. Here, I'm forging my own iron.\"", npc_meta.name)
                } else if npc_id == CitizenId(1) && is_fraternal_matured {
                    format!("{} nods as you approach the bar. \"Tomas is holding up at the woodlot, but timber prices haven't settled since Runn moved to the forge. Good to see you.\"", npc_meta.name)
                } else if npc_id != CitizenId(6) && has_heard_tomas_gossip {
                    format!("{} smiles at you. \"Tomas told me what you did at the woodlot with that great oak. We can always use good hands around here.\"", npc_meta.name)
                } else {
                    match bond.mode() {
                        BehavioralMode::DevotedAlly => {
                            format!(
                                "{} beams warmly. \"Always a delight to see you, my friend!\"",
                                npc_meta.name
                            )
                        }
                        BehavioralMode::AffectionateRefusal => {
                            format!("{} gives you an affectionate smile, though their eyes remain cautious. \"Good to see you. What's on your mind?\"", npc_meta.name)
                        }
                        BehavioralMode::GrudgingDebtor => {
                            format!("{} gives you a stiff, guarded nod. \"I acknowledge you. Let's make this brief.\"", npc_meta.name)
                        }
                        BehavioralMode::HardenedEnemy => {
                            format!(
                                "{} glares at you with cold disdain. \"What do you want?\"",
                                npc_meta.name
                            )
                        }
                        BehavioralMode::WaryConsultant => {
                            if rel >= 20 {
                                format!("{} smiles. \"Good to see you again.\"", npc_meta.name)
                            } else if rel >= 0 {
                                format!("{} nods. \"Hello.\"", npc_meta.name)
                            } else if rel >= -20 {
                                format!(
                                    "{} gives you a measured look. \"What do you want?\"",
                                    npc_meta.name
                                )
                            } else {
                                format!(
                                    "{} turns away briefly before answering. \"Well?\"",
                                    npc_meta.name
                                )
                            }
                        }
                    }
                };
                (response, vec![], 2i16)
            }
        }

        TalkTopic::AskAboutWork => {
            let work_desc = match occ.occupation {
                OccupationType::Innkeeper => format!(
                    "{} describes managing the inn — food, lodging, keeping the common room civil.",
                    npc_meta.name
                ),
                OccupationType::Artisan => format!(
                    "{} talks about the forge, about iron and charcoal and the patience required.",
                    npc_meta.name
                ),
                OccupationType::Farmer => {
                    let _ = player_epistemic.learn(2, tick);
                    format!("{} talks about the fields, the seasons, which crops do well in what weather.", npc_meta.name)
                }
                OccupationType::Herbalist => {
                    // Gain herb knowledge
                    let _ = knowledge_inv.learn(knowledge::HERB_LOCATIONS);
                    let _ = player_epistemic.learn(3, tick);
                    format!("{} shows you where the herbs grow. You learn something about local plants.", npc_meta.name)
                }
                OccupationType::Elder => {
                    // Elder hints at the archive
                    let new = knowledge_inv.learn(knowledge::ANCIENT_ARCHIVE);
                    let _ = player_epistemic.learn(4, tick);
                    let extra = if new {
                        " They mention an old archive nearby, then go quiet."
                    } else {
                        ""
                    };
                    format!(
                        "{} speaks of the settlement's history.{}",
                        npc_meta.name, extra
                    )
                }
                OccupationType::Forester => {
                    let _ = knowledge_inv.learn(knowledge::TIMBER_SOURCES);
                    let _ = player_epistemic.learn(1, tick);
                    format!(
                        "{} points out where to find good timber in the forest.",
                        npc_meta.name
                    )
                }
                _ => format!("{} explains their work in general terms.", npc_meta.name),
            };
            (work_desc, vec![], 3i16)
        }

        TalkTopic::RequestWork => {
            // Check Delia's debt leverage first (AC-204)
            let delia_debt_leveraged = npc_id == CitizenId(7) && bond.obligation >= 60;

            if delia_debt_leveraged {
                (
                    format!(
                        "{} lowers her voice, looking around nervously. \"Fine. I'll give you ledger work and market concession—just keep your silence about my debt.\"",
                        npc_meta.name
                    ),
                    vec![],
                    2i16,
                )
            } else {
                let has_felling_memory = episodic_query.iter().any(|(m, mem, _, _)| {
                    m.id == npc_id && mem.has_anchor_with_tag(MemoryTag::HelpedWithFelling)
                });
                let has_tomas_gossip = episodic_query.iter().any(|(m, mem, _, _)| {
                    m.id == npc_id
                        && mem.has_record_with_tag(MemoryTag::HeardGossipAbout(CitizenId(6)))
                });

                match bond.mode() {
                    BehavioralMode::GrudgingDebtor => {
                        // Begrudging compliance despite dislike; reduces obligation upon providing assistance
                        adjust_canonical_relationship(npc_id, 0, 0, -20, episodic_query);
                        (
                            format!(
                                "{} grimaces, looking at you with open resentment. \"I don't like you, but I honor my debts. Fine, take the work.\"",
                                npc_meta.name
                            ),
                            vec![],
                            1i16,
                        )
                    }
                    BehavioralMode::AffectionateRefusal => {
                        // High sentiment, low trust: warm refusal of sensitive assistance
                        (
                            format!(
                                "{} offers a warm, apologetic smile. \"I'm fond of you, truly. But I cannot trust you with this work right now.\"",
                                npc_meta.name
                            ),
                            vec![],
                            0i16,
                        )
                    }
                    BehavioralMode::DevotedAlly => {
                        (
                            format!(
                                "{} beams with genuine delight. \"Always a pleasure to work beside you, my friend. Let's get to it.\"",
                                npc_meta.name
                            ),
                            vec![],
                            2i16,
                        )
                    }
                    BehavioralMode::HardenedEnemy => {
                        (
                            format!(
                                "{} glares with cold hostility. \"Get away from me. I'd burn my tools before hiring you.\"",
                                npc_meta.name
                            ),
                            vec![],
                            0i16,
                        )
                    }
                    BehavioralMode::WaryConsultant => {
                        if has_felling_memory {
                            (
                                format!(
                                    "{} nods with deep respect. \"After what you did with the great oak, my work is always open to you.\"",
                                    npc_meta.name
                                ),
                                vec![],
                                2i16,
                            )
                        } else if has_tomas_gossip {
                            (
                                format!(
                                    "{} nods thoughtfully. \"Tomas told me you know your way around hard labor. I can use hands like yours.\"",
                                    npc_meta.name
                                ),
                                vec![],
                                2i16,
                            )
                        } else if bond.trust >= 0 {
                            (
                                format!(
                                    "{} looks you over. \"I could use some help. Come back when you're ready to work.\"",
                                    npc_meta.name
                                ),
                                vec![],
                                1i16,
                            )
                        } else {
                            (
                                format!(
                                    "{} shakes their head. \"Not from you. Not right now.\"",
                                    npc_meta.name
                                ),
                                vec![],
                                0i16,
                            )
                        }
                    }
                }
            }
        }

        TalkTopic::AskAboutTransformation => {
            // Elder Voss only
            if npc_id.0 == 5 {
                let elder_rel = rel;
                if bond.mode() == BehavioralMode::AffectionateRefusal {
                    ("Elder Voss smiles gently but shakes his head. 'Fond as I am of your company, the ancient archive demands absolute trust I cannot yet bestow.'".to_string(), vec![], 0i16)
                } else if (bond.trust > 40 || elder_rel > 40)
                    && knowledge_inv.knows(knowledge::ANCIENT_ARCHIVE)
                {
                    let new1 = knowledge_inv.learn(knowledge::ELDER_VOSS_SECRET);
                    let new2 = knowledge_inv.learn(knowledge::INSCRIPTION_PRIMER);
                    let msg = if new1 || new2 {
                        "Elder Voss looks at you for a long time. Then he reaches into his coat and withdraws a slim, worn book. 'The old archive was mine to tend,' he says. 'I stopped when no one was interested. Perhaps you should start.' He hands you a copy of the Inscription Primer.".to_string()
                    } else {
                        "Elder Voss nods slowly. 'You have the primer now. The rest is practice.'"
                            .to_string()
                    };
                    (
                        msg,
                        vec![
                            SideEffect::KnowledgeGained {
                                node: knowledge::ELDER_VOSS_SECRET,
                            },
                            SideEffect::KnowledgeGained {
                                node: knowledge::INSCRIPTION_PRIMER,
                            },
                        ],
                        8i16,
                    )
                } else if bond.trust > 20 || elder_rel > 20 {
                    ("Elder Voss looks at you thoughtfully. 'There are old ways of knowing this settlement that most have forgotten. Come talk to me when you know the archive.' He says nothing more.".to_string(), vec![], 3i16)
                } else {
                    ("Elder Voss gives you a long, measuring look. 'Perhaps another time.' He moves on.".to_string(), vec![], 0i16)
                }
            } else {
                (
                    format!(
                        "{} seems puzzled by the question. They don't know what you mean.",
                        npc_meta.name
                    ),
                    vec![],
                    0i16,
                )
            }
        }

        TalkTopic::AskAbout { subject } => {
            // Gain knowledge of a person
            let subj_def = content
                .npc_definitions
                .iter()
                .find(|d| CitizenId(d.citizen_id) == *subject);
            if let Some(sdef) = subj_def {
                let response = if bond.mode() == BehavioralMode::HardenedEnemy
                    || bond.mode() == BehavioralMode::AffectionateRefusal
                {
                    format!(
                        "{} gives you a guarded look. \"I don't share others' affairs with you.\"",
                        npc_meta.name
                    )
                } else if rel >= 10 || bond.trust >= 20 {
                    format!(
                        "{} tells you what they know about {}. You learn something useful.",
                        npc_meta.name, sdef.name
                    )
                } else {
                    format!("{} shrugs. \"Don't know much about them.\"", npc_meta.name)
                };
                (response, vec![], 1i16)
            } else {
                (
                    format!("{} shrugs. \"Don't know that person.\"", npc_meta.name),
                    vec![],
                    0i16,
                )
            }
        }

        TalkTopic::AskAboutLocation { location: _ } => {
            let _loc_def = content
                .npc_definitions
                .iter()
                .find(|d| d.citizen_id == npc_id.0);
            (
                "They tell you what they know about the location.".to_string(),
                vec![],
                1i16,
            )
        }

        TalkTopic::ShareKnowledge { node } => {
            // AC-204 Fail closed unless player possesses node in epistemic state or inventory
            let player_has_node =
                player_epistemic.has_knowledge(node.0 as u16) || knowledge_inv.knows(*node);
            if !player_has_node {
                return ActionResult {
                    tick,
                    success: false,
                    message: "You don't know enough about that to share it.".to_string(),
                    side_effects: vec![],
                };
            }

            // Asymmetric leverage: Delia Croft's hidden debt (Knowledge 6)
            if npc_id == CitizenId(7) && node.0 == 6 {
                if let Some((_, _, mut npc_ledger, mut npc_epistemic)) = episodic_query
                    .iter_mut()
                    .find(|(m, _, _, _)| m.id == npc_id)
                {
                    npc_epistemic.learn(6, tick);
                    // Delia's obligation increases by 80 because player holds debt leverage (GrudgingDebtor)
                    npc_ledger.adjust(CitizenId::PLAYER, -25, 0, 80);
                    let corr = npc_epistemic.get_corroboration(6);
                    event_ring.emit(SimEvent::KnowledgeShared {
                        speaker: CitizenId::PLAYER,
                        listener: npc_id,
                        knowledge_id: 6,
                        corroboration: corr,
                        tick,
                    });
                }
                return ActionResult {
                    tick,
                    success: true,
                    message: format!(
                        "{} goes pale. 'Where did you hear about that debt...? Keep your voice down. Very well, you have your terms.'",
                        npc_meta.name
                    ),
                    side_effects: vec![SideEffect::RelationshipChanged {
                        npc: npc_id,
                        delta: 1,
                    }],
                };
            }

            if let Some((_, _, _, mut npc_epistemic)) = episodic_query
                .iter_mut()
                .find(|(m, _, _, _)| m.id == npc_id)
            {
                if npc_epistemic.learn(node.0 as u16, tick) {
                    let corr = npc_epistemic.get_corroboration(node.0 as u16);
                    event_ring.emit(SimEvent::KnowledgeShared {
                        speaker: CitizenId::PLAYER,
                        listener: npc_id,
                        knowledge_id: node.0 as u16,
                        corroboration: corr,
                        tick,
                    });
                }
            }
            (
                format!(
                    "{} listens with interest. 'That's useful to know.'",
                    npc_meta.name
                ),
                vec![],
                4i16,
            )
        }
    };

    // Apply relationship delta
    if rel_delta != 0 {
        adjust_canonical_relationship(npc_id, rel_delta as i8, rel_delta as i8, 0, episodic_query);
        event_ring.emit(SimEvent::RelationshipEvent {
            actor: CitizenId::PLAYER,
            target: npc_id,
            delta: rel_delta,
            tick,
        });
    }

    let mut final_effects = side_effects;
    if rel_delta != 0 {
        final_effects.push(SideEffect::RelationshipChanged {
            npc: npc_id,
            delta: rel_delta,
        });
    }

    ActionResult {
        tick,
        success: true,
        message,
        side_effects: final_effects,
    }
}

fn resolve_help_with_felling(
    tick: u64,
    npc_id: CitizenId,
    settlement_ref: &SettlementRef,
    event_ring: &mut EventRing,
    next_causal: &mut NextCausalId,
    consequence_reg: &mut PendingConsequenceRegistry,
    npc_query: &Query<
        (
            &CitizenMeta,
            &NpcSocialProfile,
            &NpcSchedule,
            &SettlementRef,
            &OccupationProfile,
        ),
        Without<PlayerMarker>,
    >,
    episodic_query: &mut Query<
        (
            &CitizenMeta,
            &mut EpisodicMemory,
            &mut RelationalLedger,
            &mut EpistemicState,
        ),
        Without<PlayerMarker>,
    >,
) -> ActionResult {
    if settlement_ref.current_location != LocationId(11) {
        return ActionResult {
            tick,
            success: false,
            message: "You can only assist with timber felling at West Woods.".to_string(),
            side_effects: vec![],
        };
    }

    let target_npc = npc_query
        .iter()
        .find(|(meta, _, _, _, _)| meta.id == npc_id);
    if target_npc.is_none() {
        return ActionResult {
            tick,
            success: false,
            message: "You don't see that person here.".to_string(),
            side_effects: vec![],
        };
    }

    let (meta, _disp, _sched, sref, _occ) = target_npc.unwrap();
    if sref.current_location != LocationId(11) {
        return ActionResult {
            tick,
            success: false,
            message: format!("{} isn't at the felling site right now.", meta.name),
            side_effects: vec![],
        };
    }

    let already_helped = episodic_query.iter().any(|(m, mem, _, _)| {
        m.id == npc_id && mem.has_anchor_with_tag(MemoryTag::HelpedWithFelling)
    }) || consequence_reg.consequences.iter().any(|c| {
        matches!(c.consequence_type, ConsequenceType::FraternalLaborStrain { elder, .. } if elder == npc_id)
    });

    if already_helped {
        return ActionResult {
            tick,
            success: false,
            message: format!("You have already assisted {} with the great oak felling; the consequences are already unfolding.", meta.name),
            side_effects: vec![],
        };
    }

    let causal_id = next_causal.next();
    let causal_ptr = CausalPointer {
        root_event_id: causal_id,
        parent_event_id: causal_id,
        sequence_step: 1,
    };

    event_ring.emit(SimEvent::CausalAction {
        causal: causal_ptr,
        action_name: "HelpWithFelling".to_string(),
        tick,
    });

    if let Some((_, mut mem, mut ledger, _)) = episodic_query
        .iter_mut()
        .find(|(m, _, _, _)| m.id == npc_id)
    {
        mem.add_record(EpisodicRecord {
            id: causal_id,
            tick,
            actor: CitizenId::PLAYER,
            target: Some(npc_id),
            tag: MemoryTag::HelpedWithFelling,
            delta_sentiment: 45,
            delta_trust: 45,
            delta_obligation: 20,
            is_permanent: true,
            narrative_token: 101,
            causal: Some(causal_ptr),
        });
        ledger.adjust(CitizenId::PLAYER, 45, 45, 20);
    }

    consequence_reg.register(
        causal_id,
        TriggerCondition::TimeElapsed {
            duration_ticks: 336,
        },
        ConsequenceType::FraternalLaborStrain {
            elder: CitizenId(6),
            junior: CitizenId(12),
            target_workplace: LocationId(2),
        },
        tick,
    );

    ActionResult {
        tick,
        success: true,
        message: "You spend several arduous hours helping Tomas Birch fell and clear the heavy timber at West Woods. Tomas wipes the sweat from his brow and clasps your arm in deep gratitude: 'I won't forget this. Few strangers would give their backs to another man's labor.' In the distance, young Runn watches silently, an unreadable shadow crossing his face.".to_string(),
        side_effects: vec![SideEffect::RelationshipChanged { npc: npc_id, delta: 45 }],
    }
}

fn resolve_offer(
    tick: u64,
    npc_id: CitizenId,
    exchange: &Exchange,
    finances: &mut PersonalFinances,
    _inventory: &mut Inventory,
    _capabilities: &mut CapabilitySet,
    episodic_query: &mut Query<
        (
            &CitizenMeta,
            &mut EpisodicMemory,
            &mut RelationalLedger,
            &mut EpistemicState,
        ),
        Without<PlayerMarker>,
    >,
    _settlements: &mut SettlementDirectory,
    _event_ring: &mut EventRing,
) -> ActionResult {
    let bond = episodic_query
        .iter()
        .find(|(m, _, _, _)| m.id == npc_id)
        .map(|(_, _, l, _)| l.get_bond(CitizenId::PLAYER))
        .unwrap_or_default();

    match bond.mode() {
        BehavioralMode::HardenedEnemy => {
            return ActionResult {
                tick,
                success: false,
                message: "They're not interested in doing business with you right now.".to_string(),
                side_effects: vec![],
            };
        }
        BehavioralMode::AffectionateRefusal => {
            if exchange.offer_coins < exchange.request_coins {
                return ActionResult {
                    tick,
                    success: false,
                    message:
                        "They smile warmly but decline to extend credit without greater trust."
                            .to_string(),
                    side_effects: vec![],
                };
            }
        }
        _ => {}
    }

    // Simple validation: player must have offered resources
    if exchange.offer_coins > finances.coins {
        return ActionResult {
            tick,
            success: false,
            message: "You don't have enough coins to make that offer.".to_string(),
            side_effects: vec![],
        };
    }

    // Apply coin exchange
    let mut side_effects = vec![];
    if exchange.offer_coins > 0.0 {
        finances.coins -= exchange.offer_coins;
        side_effects.push(SideEffect::CoinsLost(exchange.offer_coins));
    }
    if exchange.request_coins > 0.0 {
        finances.coins += exchange.request_coins;
        side_effects.push(SideEffect::CoinsGained(exchange.request_coins));
    }

    adjust_canonical_relationship(npc_id, 3, 3, 0, episodic_query);
    side_effects.push(SideEffect::RelationshipChanged {
        npc: npc_id,
        delta: 3,
    });

    ActionResult {
        tick,
        success: true,
        message: "The exchange is accepted.".to_string(),
        side_effects,
    }
}

fn resolve_buy(
    tick: u64,
    resource: ResourceType,
    quantity: u32,
    finances: &mut PersonalFinances,
    inventory: &mut Inventory,
    settlements: &mut SettlementDirectory,
    event_ring: &mut EventRing,
) -> ActionResult {
    let sid = SettlementDirectory::thornveil_id();
    let ordinal = resource_ordinal(resource);

    let price = settlements
        .get(sid)
        .map(|s| s.get_price(ordinal))
        .unwrap_or(99.0);
    let total_cost = price as f64 * quantity as f64;

    if finances.coins < total_cost {
        return ActionResult {
            tick,
            success: false,
            message: format!(
                "You need {:.1} coins to buy {} {} (at {:.1} each). You only have {:.1}.",
                total_cost,
                quantity,
                resource.display_name(),
                price,
                finances.coins
            ),
            side_effects: vec![],
        };
    }

    let has_stock = settlements
        .get(sid)
        .map(|s| s.get_stock(ordinal) >= quantity as f64)
        .unwrap_or(false);

    if !has_stock {
        return ActionResult {
            tick,
            success: false,
            message: format!(
                "The market doesn't have enough {} in stock right now.",
                resource.display_name()
            ),
            side_effects: vec![],
        };
    }

    // Execute transaction
    finances.coins -= total_cost;
    finances.last_expense += total_cost as f32;
    if let Some(s) = settlements.get_mut(sid) {
        s.remove_stock(ordinal, quantity as f64);
    }
    inventory.add(ordinal, quantity);

    event_ring.emit(SimEvent::EconomicTransaction {
        buyer: CitizenId::PLAYER,
        seller: CitizenId(1),
        resource,
        quantity,
        price,
        tick,
    });

    ActionResult {
        tick,
        success: true,
        message: format!(
            "You buy {} {} for {:.1} coins. You now have {:.1} coins.",
            quantity,
            resource.display_name(),
            total_cost,
            finances.coins
        ),
        side_effects: vec![
            SideEffect::CoinsLost(total_cost),
            SideEffect::ResourceGained { resource, quantity },
        ],
    }
}

fn resolve_sell(
    tick: u64,
    resource: ResourceType,
    quantity: u32,
    finances: &mut PersonalFinances,
    inventory: &mut Inventory,
    settlements: &mut SettlementDirectory,
    _event_ring: &mut EventRing,
) -> ActionResult {
    let ordinal = resource_ordinal(resource);
    let sid = SettlementDirectory::thornveil_id();

    if inventory.get(ordinal) < quantity {
        return ActionResult {
            tick,
            success: false,
            message: format!(
                "You don't have {} {} to sell.",
                quantity,
                resource.display_name()
            ),
            side_effects: vec![],
        };
    }

    let price = settlements
        .get(sid)
        .map(|s| s.get_price(ordinal))
        .unwrap_or(1.0);
    let total_earned = price as f64 * quantity as f64 * 0.85; // 15% market cut

    inventory.remove(ordinal, quantity);
    finances.coins += total_earned;
    finances.last_income += total_earned as f32;
    if let Some(s) = settlements.get_mut(sid) {
        s.add_stock(ordinal, quantity as f64);
    }

    ActionResult {
        tick,
        success: true,
        message: format!(
            "You sell {} {} for {:.1} coins (market cut applied).",
            quantity,
            resource.display_name(),
            total_earned
        ),
        side_effects: vec![
            SideEffect::CoinsGained(total_earned),
            SideEffect::ResourceLost { resource, quantity },
        ],
    }
}

fn resolve_work(
    tick: u64,
    occupation: OccupationType,
    finances: &mut PersonalFinances,
    needs: &mut PhysicalNeeds,
    inventory: &mut Inventory,
    capabilities: &mut CapabilitySet,
    settlement_ref: &SettlementRef,
    _settlements: &mut SettlementDirectory,
    event_ring: &mut EventRing,
) -> ActionResult {
    let _loc = settlement_ref.current_location;

    // Check capability requirements
    let (can_work, cap_needed, resource_earned, resource_ordinal, earn_rate) = match occupation {
        OccupationType::Farmer => (
            capabilities.has(caps::FARMING) || true, // anyone can farm at novice level
            caps::FARMING,
            ResourceType::Food,
            0u8,
            2.0f64, // coins per work session (8 ticks)
        ),
        OccupationType::Forester => (
            capabilities.has(caps::WOODCUTTING),
            caps::WOODCUTTING,
            ResourceType::Timber,
            1u8,
            3.0,
        ),
        OccupationType::Artisan => (
            capabilities.has(caps::SMITHING),
            caps::SMITHING,
            ResourceType::Tools,
            3u8,
            5.0,
        ),
        OccupationType::Herbalist => (
            capabilities.has(caps::HERBALISM),
            caps::HERBALISM,
            ResourceType::Herbs,
            5u8,
            3.5,
        ),
        OccupationType::Laborer => (
            true, // always available
            CapabilityId(0),
            ResourceType::Food,
            0u8,
            1.5,
        ),
        _ => {
            return ActionResult {
                tick,
                success: false,
                message: "That occupation isn't available to work at directly.".to_string(),
                side_effects: vec![],
            }
        }
    };

    if !can_work {
        return ActionResult {
            tick,
            success: false,
            message: format!(
                "You don't have the skill to do that work. Learn {} first.",
                cap_needed.0
            ),
            side_effects: vec![],
        };
    }

    // Work for 8 ticks = 1 work session
    // Returns coins and resource, costs rest
    let resources_gathered = 2u32;
    inventory.add(resource_ordinal, resources_gathered);
    finances.coins += earn_rate;
    needs.rest = needs.rest.saturating_sub(10);

    // Gain experience in relevant capability
    let leveled = if cap_needed.0 > 0 {
        capabilities.add_practice(cap_needed, 5)
    } else {
        false
    };

    event_ring.emit(SimEvent::PlayerAction {
        tick,
        action_name: format!("Work({:?})", occupation),
        success: true,
    });

    let level_msg = if leveled {
        " Your skill has improved!"
    } else {
        ""
    };

    ActionResult {
        tick,
        success: true,
        message: format!(
            "You work for a few hours. You earn {:.1} coins and gather {} {}.{}",
            earn_rate,
            resources_gathered,
            resource_earned.display_name(),
            level_msg
        ),
        side_effects: vec![
            SideEffect::CoinsGained(earn_rate),
            SideEffect::ResourceGained {
                resource: resource_earned,
                quantity: resources_gathered,
            },
        ],
    }
}

fn resolve_practice(
    tick: u64,
    capability: CapabilityId,
    capabilities: &mut CapabilitySet,
    content: &ContentDefinitions,
    event_ring: &mut EventRing,
    _inventory: &mut Inventory,
) -> ActionResult {
    let cap_def = content
        .capability_definitions
        .iter()
        .find(|c| c.id == capability);

    let cap_name = cap_def.map(|c| c.name.as_str()).unwrap_or("unknown skill");

    let current_level = capabilities.level(capability);
    if current_level.0 == 0 {
        // Need to learn it first (either from an NPC or discover it)
        // Exception: allow basic practice to get to novice for common skills
        capabilities.set(capability, CapabilityLevel::NOVICE);
        capabilities.practice_progress.insert(capability.0, 10);

        event_ring.emit(SimEvent::CapabilityAcquired {
            citizen: CitizenId::PLAYER,
            capability_id: capability.0,
            tick,
        });

        return ActionResult {
            tick,
            success: true,
            message: format!(
                "You begin practicing {}. You're a novice, but it's a start.",
                cap_name
            ),
            side_effects: vec![SideEffect::CapabilityGained {
                capability,
                level: CapabilityLevel::NOVICE,
            }],
        };
    }

    let leveled = capabilities.add_practice(capability, 10);
    let new_level = capabilities.level(capability);

    if leveled {
        event_ring.emit(SimEvent::CapabilityAcquired {
            citizen: CitizenId::PLAYER,
            capability_id: capability.0,
            tick,
        });
        ActionResult {
            tick,
            success: true,
            message: format!("Your {} has improved to {}!", cap_name, new_level.display()),
            side_effects: vec![SideEffect::CapabilityGained {
                capability,
                level: new_level,
            }],
        }
    } else {
        let progress = capabilities
            .practice_progress
            .get(&capability.0)
            .copied()
            .unwrap_or(0);
        ActionResult {
            tick,
            success: true,
            message: format!(
                "You practice {}. Progress: {}/100 toward {}.",
                cap_name, progress, "next level"
            ),
            side_effects: vec![],
        }
    }
}

fn resolve_learn_from(
    tick: u64,
    npc_id: CitizenId,
    capability: CapabilityId,
    capabilities: &mut CapabilitySet,
    _knowledge_inv: &mut KnowledgeInventory,
    event_ring: &mut EventRing,
    npc_query: &Query<
        (
            &CitizenMeta,
            &NpcSocialProfile,
            &NpcSchedule,
            &SettlementRef,
            &OccupationProfile,
        ),
        Without<PlayerMarker>,
    >,
    content: &ContentDefinitions,
    episodic_query: &mut Query<
        (
            &CitizenMeta,
            &mut EpisodicMemory,
            &mut RelationalLedger,
            &mut EpistemicState,
        ),
        Without<PlayerMarker>,
    >,
) -> ActionResult {
    let npc_data = npc_query.iter().find(|(m, _, _, _, _)| m.id == npc_id);
    if npc_data.is_none() {
        return ActionResult {
            tick,
            success: false,
            message: "That person isn't here.".to_string(),
            side_effects: vec![],
        };
    }

    let (npc_meta, social_profile, _, _, _) = npc_data.unwrap();

    // Check if NPC can teach this capability
    let can_teach = social_profile.will_teach == Some(capability);
    if !can_teach {
        return ActionResult {
            tick,
            success: false,
            message: format!("{} doesn't know how to teach you that.", npc_meta.name),
            side_effects: vec![],
        };
    }

    // Check relationship mode and threshold (AC-201)
    let bond = episodic_query
        .iter()
        .find(|(m, _, _, _)| m.id == npc_id)
        .map(|(_, _, l, _)| l.get_bond(CitizenId::PLAYER))
        .unwrap_or_default();

    match bond.mode() {
        BehavioralMode::HardenedEnemy => {
            return ActionResult {
                tick,
                success: false,
                message: format!(
                    "{} refuses to speak with you, let alone share their craft.",
                    npc_meta.name
                ),
                side_effects: vec![],
            };
        }
        BehavioralMode::AffectionateRefusal => {
            return ActionResult {
                tick,
                success: false,
                message: format!(
                    "{} smiles gently. \"I care for you, but teaching this craft requires deep trust I cannot offer yet.\"",
                    npc_meta.name
                ),
                side_effects: vec![],
            };
        }
        BehavioralMode::GrudgingDebtor => {
            // Complies to pay off obligation; reduces obligation by 30 upon teaching
            adjust_canonical_relationship(npc_id, 0, 0, -30, episodic_query);
        }
        BehavioralMode::DevotedAlly => {
            // Devoted ally willingly teaches
        }
        BehavioralMode::WaryConsultant => {
            let threshold = social_profile.teach_threshold;
            let rel = canonical_relationship_score(npc_id, episodic_query);
            if rel < threshold {
                let rel_needed = threshold - rel;
                return ActionResult {
                    tick,
                    success: false,
                    message: format!(
                        "{} isn't ready to teach you yet. Your relationship needs to improve by {} more.",
                        npc_meta.name, rel_needed
                    ),
                    side_effects: vec![],
                };
            }
        }
    }

    // Teaching happens
    let cap_def = content
        .capability_definitions
        .iter()
        .find(|c| c.id == capability);
    let cap_name = cap_def.map(|c| c.name.as_str()).unwrap_or("this skill");

    capabilities.set(capability, CapabilityLevel::NOVICE);
    capabilities.practice_progress.insert(capability.0, 30); // head start from teaching

    event_ring.emit(SimEvent::CapabilityAcquired {
        citizen: CitizenId::PLAYER,
        capability_id: capability.0,
        tick,
    });

    // Small relationship boost from shared experience
    adjust_canonical_relationship(npc_id, 5, 5, 0, episodic_query);

    ActionResult {
        tick,
        success: true,
        message: format!(
            "{} agrees to teach you {}. You spend time learning together. You are now a Novice.",
            npc_meta.name, cap_name
        ),
        side_effects: vec![
            SideEffect::CapabilityGained {
                capability,
                level: CapabilityLevel::NOVICE,
            },
            SideEffect::RelationshipChanged {
                npc: npc_id,
                delta: 5,
            },
        ],
    }
}

fn resolve_inscribe(
    tick: u64,
    observation: &str,
    transform: &mut TransformationState,
    capabilities: &mut CapabilitySet,
    _knowledge_inv: &mut KnowledgeInventory,
    event_ring: &mut EventRing,
) -> ActionResult {
    if !capabilities.has(caps::INSCRIPTION) {
        return ActionResult {
            tick,
            success: false,
            message: "You don't know how to inscribe observations in a structured form. You need to learn Inscription first.".to_string(),
            side_effects: vec![],
        };
    }

    if observation.trim().is_empty() {
        return ActionResult {
            tick,
            success: false,
            message: "What do you want to inscribe? Provide an observation.".to_string(),
            side_effects: vec![],
        };
    }

    transform.inscriptions_completed += 1;
    let count = transform.inscriptions_completed;

    let milestone_msg = if count == 5 {
        "\n\nYou've now inscribed five observations. The habit of recording is becoming part of how you see the world."
    } else {
        ""
    };

    event_ring.emit(SimEvent::TransformationEvent {
        stage: transform.stage,
        tick,
    });

    ActionResult {
        tick,
        success: true,
        message: format!(
            "You inscribe your observation: \"{}\"\n\nInscription #{} complete.{}",
            observation.chars().take(80).collect::<String>(),
            count,
            milestone_msg
        ),
        side_effects: vec![SideEffect::TransformationProgress {
            stage: transform.stage,
            progress: transform.progress + 5,
        }],
    }
}

fn resolve_study_archive(
    tick: u64,
    transform: &mut TransformationState,
    capabilities: &mut CapabilitySet,
    knowledge_inv: &mut KnowledgeInventory,
    settlement_ref: &SettlementRef,
    _event_ring: &mut EventRing,
    _content: &ContentDefinitions,
) -> ActionResult {
    if settlement_ref.current_location != LocationId(8) {
        return ActionResult {
            tick,
            success: false,
            message: "You need to be at The Old Archive to study it.".to_string(),
            side_effects: vec![],
        };
    }

    // Gain archive knowledge on first visit
    let new_knowledge = knowledge_inv.learn(knowledge::ANCIENT_ARCHIVE);
    if new_knowledge {
        return ActionResult {
            tick,
            success: true,
            message: "You examine the ruined archive. Most of the records are gone, but the structure itself is interesting — it was clearly designed to store knowledge systematically. You learn something about the archive's history.".to_string(),
            side_effects: vec![
                SideEffect::KnowledgeGained { node: knowledge::ANCIENT_ARCHIVE },
            ],
        };
    }

    if !capabilities.has(caps::INSCRIPTION) {
        return ActionResult {
            tick,
            success: true,
            message: "You look through what remains. Without knowing how to read the inscription format, most of it is meaningless to you. Perhaps someone in Thornveil knows this system.".to_string(),
            side_effects: vec![],
        };
    }

    // Study with inscription capability
    transform.archive_studied_count += 1;
    let count = transform.archive_studied_count;

    ActionResult {
        tick,
        success: true,
        message: format!(
            "You study the archive fragments. With your inscription knowledge, you can read parts of what remains. Study session #{}. The fragments reveal hints of a larger system that once existed here.",
            count
        ),
        side_effects: vec![
            SideEffect::TransformationProgress { stage: transform.stage, progress: transform.progress + 3 },
        ],
    }
}

fn resolve_sleep(
    tick: u64,
    needs: &mut PhysicalNeeds,
    settlement_ref: &SettlementRef,
    finances: &mut PersonalFinances,
    _settlements: &mut SettlementDirectory,
) -> ActionResult {
    let loc = settlement_ref.current_location;

    // Inn costs money to sleep (if player is at the inn)
    let inn_cost = if loc == LocationId(1) { 2.0 } else { 0.0 };

    if inn_cost > 0.0 && finances.coins < inn_cost {
        return ActionResult {
            tick,
            success: false,
            message: format!(
                "You can't afford to sleep at the inn ({:.1} coins).",
                inn_cost
            ),
            side_effects: vec![],
        };
    }

    if inn_cost > 0.0 {
        finances.coins -= inn_cost;
        finances.last_expense += inn_cost as f32;
    }

    // Restore rest and some satiety from sleep
    needs.rest = (needs.rest + 60).min(100);
    needs.satiety = (needs.satiety + 15).min(100); // small meal assumed

    let inn_msg = if inn_cost > 0.0 {
        format!(" (cost {:.1} coins)", inn_cost)
    } else {
        " (you rest where you can)".to_string()
    };

    ActionResult {
        tick,
        success: true,
        message: format!("You sleep for several hours{}, waking rested.", inn_msg),
        side_effects: if inn_cost > 0.0 {
            vec![SideEffect::CoinsLost(inn_cost)]
        } else {
            vec![]
        },
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

pub fn resource_ordinal(r: ResourceType) -> u8 {
    match r {
        ResourceType::Food => 0,
        ResourceType::Timber => 1,
        ResourceType::Stone => 2,
        ResourceType::Tools => 3,
        ResourceType::Luxury => 4,
        ResourceType::Herbs => 5,
        ResourceType::Ink => 6,
        ResourceType::Parchment => 7,
    }
}

fn resolve_diagnose(
    tick: u64,
    location: LocationId,
    capabilities: &CapabilitySet,
    transform: &TransformationState,
    settlement_ref: &SettlementRef,
    player_epistemic: &mut EpistemicState,
    event_ring: &mut EventRing,
) -> ActionResult {
    if !capabilities.has(caps::DIAGNOSIS) && transform.stage < 2 {
        return ActionResult {
            tick,
            success: false,
            message: "You lack the diagnostic training to analyze systemic settlement conditions. Reach Scholar Stage 2 (The Settlement Chronicler) first.".to_string(),
            side_effects: vec![],
        };
    }

    if settlement_ref.current_location != location {
        return ActionResult {
            tick,
            success: false,
            message: format!(
                "You are not at Location #{}. You must be on-site to conduct a diagnosis.",
                location.0
            ),
            side_effects: vec![],
        };
    }

    let (_finding_id, report_text) = match location.0 {
        5 => {
            player_epistemic.learn(2, tick);
            (
                2u16,
                "Agricultural Diagnosis (South Fields): Stalk mildew, soil drainage pooling, and necrotic rust in the lower furrows. The crops suffer from fungal blight vulnerability."
            )
        }
        4 => {
            player_epistemic.learn(2, tick);
            (
                2u16,
                "Agricultural Diagnosis (North Fields): Soil nitrogen depletion and weed encroachment along the boundary furrows."
            )
        }
        11 => {
            player_epistemic.learn(1, tick);
            (
                1u16,
                "Silvicultural Diagnosis (Forest Edge): Fungal conks, heart rot, and crown thinning in the mature oak stands. The timber shows severe environmental stress."
            )
        }
        8 => {
            player_epistemic.learn(4, tick);
            (
                4u16,
                "Archaeological Diagnosis (The Old Archive): Moisture-sealed subterranean vault joints and foundational stone settlement beneath the collapsed nave."
            )
        }
        6 => {
            player_epistemic.learn(3, tick);
            (
                3u16,
                "Botanical Diagnosis (Herb Garden): Silverleaf moss and mountain sage have adapted to limestone shale along the shaded western wall."
            )
        }
        _ => {
            (
                0u16,
                "Survey Diagnosis: You observe general structural wear and municipal foot traffic patterns."
            )
        }
    };

    event_ring.emit(SimEvent::PlayerAction {
        tick,
        action_name: format!("Diagnose(LocationId({}))", location.0),
        success: true,
    });

    ActionResult {
        tick,
        success: true,
        message: format!(
            "You conduct a methodical on-site examination.\n\n{}",
            report_text
        ),
        side_effects: vec![],
    }
}

fn resolve_draft_document(
    tick: u64,
    doc_type: &DocumentType,
    capabilities: &CapabilitySet,
    transform: &TransformationState,
    player_epistemic: &EpistemicState,
    documents: &mut DocumentRegistry,
    event_ring: &mut EventRing,
) -> ActionResult {
    if !capabilities.has(caps::DIAGNOSIS)
        && transform.stage < 2
        && capabilities.level(caps::INSCRIPTION) < CapabilityLevel::JOURNEYMAN
    {
        return ActionResult {
            tick,
            success: false,
            message: "You lack documentary authority. You must reach Scholar Stage 2 (The Settlement Chronicler) to draft legally binding instruments.".to_string(),
            side_effects: vec![],
        };
    }

    match doc_type {
        DocumentType::HarvestDiagnosisReport {
            location: _,
            finding,
        } => {
            if *finding > 0 && !player_epistemic.has_knowledge(*finding) {
                return ActionResult {
                    tick,
                    success: false,
                    message: format!("You have not diagnosed or observed finding #{} yet. You cannot draft an unsubstantiated report.", finding),
                    side_effects: vec![],
                };
            }
        }
        DocumentType::DebtReliefCharter {
            creditor,
            debtor,
            terms,
        } => {
            if creditor == debtor || *terms == 0 {
                return ActionResult {
                    tick,
                    success: false,
                    message: "Invalid debt relief charter parameters.".to_string(),
                    side_effects: vec![],
                };
            }
        }
        DocumentType::FoundingArchiveTranslation { secret_id } => {
            if *secret_id > 0 && !player_epistemic.has_knowledge(*secret_id) {
                return ActionResult {
                    tick,
                    success: false,
                    message:
                        "You cannot translate an archive secret you have not observed or learned."
                            .to_string(),
                    side_effects: vec![],
                };
            }
        }
    }

    let doc = InscribedDocument {
        id: 0,
        doc_type: doc_type.clone(),
        drafter: CitizenId::PLAYER,
        signers: vec![CitizenId::PLAYER],
        binding_tick: tick,
        related_consequence_id: None,
    };

    let doc_id = documents.register(doc);

    event_ring.emit(SimEvent::PlayerAction {
        tick,
        action_name: format!("DraftDocument(#{})", doc_id),
        success: true,
    });

    ActionResult {
        tick,
        success: true,
        message: format!(
            "Using iron gall ink and official parchment, you draft and seal:\n\"{}\"\n\nInscribed Document #{} has been registered with documentary authority.",
            doc_type.title(),
            doc_id
        ),
        side_effects: vec![SideEffect::DocumentCreated { id: doc_id }],
    }
}

fn resolve_arbitrate_dispute(
    tick: u64,
    document_id: u32,
    consequence_id: u32,
    documents: &mut DocumentRegistry,
    consequences: &mut PendingConsequenceRegistry,
    episodic_query: &mut Query<
        (
            &CitizenMeta,
            &mut EpisodicMemory,
            &mut RelationalLedger,
            &mut EpistemicState,
        ),
        Without<PlayerMarker>,
    >,
    next_causal: &mut NextCausalId,
    event_ring: &mut EventRing,
) -> ActionResult {
    let consequence = if let Some(c) = consequences
        .consequences
        .iter_mut()
        .find(|c| c.id == consequence_id)
    {
        c
    } else {
        return ActionResult {
            tick,
            success: false,
            message: format!("Pending consequence #{} not found.", consequence_id),
            side_effects: vec![],
        };
    };

    let (doc_title, doc_matches) = if let Some(doc) = documents.get(document_id) {
        let matches = match (&consequence.consequence_type, &doc.doc_type) {
            (
                ConsequenceType::CropBlightDispute { .. },
                DocumentType::HarvestDiagnosisReport { .. },
            ) => true,
            (ConsequenceType::DebtDispute { .. }, DocumentType::DebtReliefCharter { .. }) => true,
            (
                ConsequenceType::FraternalLaborStrain { .. },
                DocumentType::FoundingArchiveTranslation { .. },
            ) => true,
            _ => false,
        };
        (doc.doc_type.title(), matches)
    } else {
        return ActionResult {
            tick,
            success: false,
            message: format!(
                "Document #{} does not exist in official records.",
                document_id
            ),
            side_effects: vec![],
        };
    };

    if !doc_matches {
        return ActionResult {
            tick,
            success: false,
            message: format!(
                "Document #{} ({}) is not legally applicable to arbitrate this dispute.",
                document_id, doc_title
            ),
            side_effects: vec![],
        };
    }

    if consequence.stage == ConsequenceStage::Resolved {
        return ActionResult {
            tick,
            success: false,
            message: format!(
                "Situation #{} has already been peacefully resolved.",
                consequence_id
            ),
            side_effects: vec![],
        };
    }

    consequence.stage = ConsequenceStage::Resolved;
    let causal_root = consequence.causal_root;

    let mut involved_citizens = Vec::new();
    match &consequence.consequence_type {
        ConsequenceType::FraternalLaborStrain { elder, junior, .. } => {
            involved_citizens.push(*elder);
            involved_citizens.push(*junior);
        }
        ConsequenceType::CropBlightDispute {
            farmer_a, farmer_b, ..
        } => {
            involved_citizens.push(*farmer_a);
            involved_citizens.push(*farmer_b);
        }
        ConsequenceType::DebtDispute {
            creditor, debtor, ..
        } => {
            involved_citizens.push(*creditor);
            involved_citizens.push(*debtor);
        }
    }

    if let Some(doc_mut) = documents.get_mut(document_id) {
        doc_mut.related_consequence_id = Some(consequence_id);
        for &cit in &involved_citizens {
            if !doc_mut.signers.contains(&cit) {
                doc_mut.signers.push(cit);
            }
        }
    }

    for &cit in &involved_citizens {
        for (meta, mut mem, mut ledger, _) in episodic_query.iter_mut() {
            if meta.id == cit {
                let turn_id = next_causal.next();
                mem.add_record(EpisodicRecord {
                    id: turn_id,
                    tick,
                    actor: CitizenId::PLAYER,
                    target: Some(cit),
                    tag: MemoryTag::ContractSigned,
                    delta_sentiment: 20,
                    delta_trust: 35,
                    delta_obligation: 40,
                    is_permanent: true,
                    narrative_token: 205,
                    causal: Some(CausalPointer {
                        root_event_id: causal_root,
                        parent_event_id: causal_root,
                        sequence_step: 3,
                    }),
                });

                let bond = ledger
                    .bonds
                    .entry(0)
                    .or_insert_with(RelationalBond::default);
                *bond = RelationalBond::new(
                    (bond.sentiment + 25).min(100),
                    (bond.trust + 40).min(100),
                    (bond.obligation + 50).min(1000),
                );
            }
        }
    }

    if involved_citizens.len() >= 2 {
        let a = involved_citizens[0];
        let b = involved_citizens[1];
        for (meta, _, mut ledger, _) in episodic_query.iter_mut() {
            if meta.id == a {
                let bond_b = ledger
                    .bonds
                    .entry(b.0)
                    .or_insert_with(RelationalBond::default);
                *bond_b = RelationalBond::new(
                    (bond_b.sentiment + 20).max(10).min(100),
                    (bond_b.trust + 20).max(10).min(100),
                    0,
                );
            } else if meta.id == b {
                let bond_a = ledger
                    .bonds
                    .entry(a.0)
                    .or_insert_with(RelationalBond::default);
                *bond_a = RelationalBond::new(
                    (bond_a.sentiment + 20).max(10).min(100),
                    (bond_a.trust + 20).max(10).min(100),
                    0,
                );
            }
        }
    }

    event_ring.emit(SimEvent::PlayerAction {
        tick,
        action_name: format!(
            "ArbitrateDispute(Doc#{}, Consequence#{})",
            document_id, consequence_id
        ),
        success: true,
    });

    ActionResult {
        tick,
        success: true,
        message: format!(
            "You present Inscribed Document #{} (\"{}\") with the full documentary authority of a Settlement Chronicler.\n\nAll parties review the binding provisions and affix their marks. Situation #{} is formally RESOLVED. Feud and division are averted.",
            document_id, doc_title, consequence_id
        ),
        side_effects: vec![SideEffect::DisputeArbitrated { consequence_id, document_id }],
    }
}
