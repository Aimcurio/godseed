/// Godseed — Content Definitions
///
/// Authored content for Thornveil: NPC identities, capabilities, knowledge nodes,
/// transformation path definition. Systems are generic; content provides identity.

use bevy_ecs::system::Resource;
use serde::{Deserialize, Serialize};

use crate::types::{
    BehavioralMode, CapabilityId, Gender, HouseholdRole,
    KnowledgeDefinition, KnowledgeDomain, KnowledgeNode, LocationId, NpcActivity,
    OccupationType, ResourceType, ScheduleSlot,
};

// ── Capability Definitions ────────────────────────────────────────────────────

/// Well-known capability IDs (canonical constants)
pub mod caps {
    use crate::types::CapabilityId;
    pub const WOODCUTTING: CapabilityId = CapabilityId(1);
    pub const SMITHING: CapabilityId = CapabilityId(2);
    pub const PERSUASION: CapabilityId = CapabilityId(3);
    pub const HERBALISM: CapabilityId = CapabilityId(4);
    pub const COOKING: CapabilityId = CapabilityId(5);
    pub const TRACKING: CapabilityId = CapabilityId(6);
    pub const INSCRIPTION: CapabilityId = CapabilityId(7);
    pub const FARMING: CapabilityId = CapabilityId(8);
    pub const TRADING: CapabilityId = CapabilityId(9);
    pub const DIAGNOSIS: CapabilityId = CapabilityId(10);
}

/// Well-known knowledge node IDs
pub mod knowledge {
    use crate::types::KnowledgeNodeId;
    pub const ANCIENT_ARCHIVE: KnowledgeNodeId = KnowledgeNodeId(1);
    pub const ELDER_VOSS_SECRET: KnowledgeNodeId = KnowledgeNodeId(2);
    pub const INSCRIPTION_PRIMER: KnowledgeNodeId = KnowledgeNodeId(3);
    pub const HERB_LOCATIONS: KnowledgeNodeId = KnowledgeNodeId(4);
    pub const TIMBER_SOURCES: KnowledgeNodeId = KnowledgeNodeId(5);
    pub const MIRA_HISTORY: KnowledgeNodeId = KnowledgeNodeId(6);
    pub const WREN_GRUDGE: KnowledgeNodeId = KnowledgeNodeId(7);
    pub const SETTLEMENT_FOUNDING: KnowledgeNodeId = KnowledgeNodeId(8);
}

/// Milestone IDs for the Inscription path
pub mod milestones {
    use crate::types::MilestoneId;
    pub const ARCHIVE_DISCOVERED: MilestoneId = MilestoneId(1);
    pub const ELDER_CONSULTED: MilestoneId = MilestoneId(2);
    pub const INSCRIPTION_LEARNED: MilestoneId = MilestoneId(3);
    pub const FIVE_INSCRIPTIONS: MilestoneId = MilestoneId(4);
    pub const SCHOLAR_RECOGNIZED: MilestoneId = MilestoneId(5);
    pub const CHRONICLER_RECOGNIZED: MilestoneId = MilestoneId(6);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityDefinition {
    pub id: CapabilityId,
    pub name: String,
    pub description: String,
    pub practice_resource: Option<ResourceType>,
    pub unlocks_actions: Vec<String>,
}

// ── NPC Definitions ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcDefinition {
    pub citizen_id: u64, // CitizenId.0 (1-indexed; 0 is reserved for player)
    pub name: String,
    pub gender: Gender,
    pub age: u16,
    pub occupation: OccupationType,
    pub household_id: u32,
    pub household_role: HouseholdRole,
    pub home_location: LocationId,
    pub work_location: LocationId,
    pub base_personality: i8, // -20 to +20; initial disposition toward strangers
    pub will_teach: Option<CapabilityId>,
    pub teach_threshold: i16,
    pub starting_coins: f64,
    pub description: String, // What the player sees on Inspect
    pub schedule: Vec<ScheduleSlot>,
}

// ── Content Definitions Resource ──────────────────────────────────────────────

#[derive(Resource, Debug, Clone)]
pub struct ContentDefinitions {
    pub npc_definitions: Vec<NpcDefinition>,
    pub capability_definitions: Vec<CapabilityDefinition>,
    pub knowledge_nodes: Vec<KnowledgeNode>,
    pub knowledge_definitions: Vec<KnowledgeDefinition>,
}

impl ContentDefinitions {
    pub fn thornveil() -> Self {
        Self {
            npc_definitions: thornveil_npcs(),
            capability_definitions: capability_defs(),
            knowledge_nodes: knowledge_nodes(),
            knowledge_definitions: knowledge_definitions(),
        }
    }
}

fn knowledge_definitions() -> Vec<KnowledgeDefinition> {
    vec![
        KnowledgeDefinition {
            id: 1,
            domain: KnowledgeDomain::ObservationInsight,
            title: "Timber Stress Signs".to_string(),
            description: "Heart rot and heavy fungal spread in the old stand at West Woods.".to_string(),
            social_fallout_mode: None,
        },
        KnowledgeDefinition {
            id: 2,
            domain: KnowledgeDomain::ObservationInsight,
            title: "Crop Blight Vulnerability".to_string(),
            description: "Stalk mildew threatening the lower furrows of South Fields during damp weeks.".to_string(),
            social_fallout_mode: None,
        },
        KnowledgeDefinition {
            id: 3,
            domain: KnowledgeDomain::ObservationInsight,
            title: "Herb Habitats".to_string(),
            description: "Rare silverleaf moss flourishing in the sheltered shale behind the garden wall.".to_string(),
            social_fallout_mode: None,
        },
        KnowledgeDefinition {
            id: 4,
            domain: KnowledgeDomain::ObservationInsight,
            title: "Ancient Archive Lore".to_string(),
            description: "A subterranean archive vault buried beneath the collapsed stone nave.".to_string(),
            social_fallout_mode: Some(BehavioralMode::WaryConsultant),
        },
        KnowledgeDefinition {
            id: 5,
            domain: KnowledgeDomain::SecretTruth,
            title: "Elder Voss's Exiled Son".to_string(),
            description: "Voss's eldest blood kin was quietly banished past the ridge thirty winters ago.".to_string(),
            social_fallout_mode: Some(BehavioralMode::AffectionateRefusal),
        },
        KnowledgeDefinition {
            id: 6,
            domain: KnowledgeDomain::SecretTruth,
            title: "Delia's Hidden Debt".to_string(),
            description: "Delia owes seventy silver guild coins under a private penalty contract.".to_string(),
            social_fallout_mode: Some(BehavioralMode::GrudgingDebtor),
        },
        KnowledgeDefinition {
            id: 7,
            domain: KnowledgeDomain::DocumentedRecord,
            title: "Founding Land Charter".to_string(),
            description: "The original sealed parchment establishing Thornveil's ancient pasture boundaries.".to_string(),
            social_fallout_mode: Some(BehavioralMode::WaryConsultant),
        },
    ]
}

// ── Thornveil NPC Roster ──────────────────────────────────────────────────────

fn thornveil_npcs() -> Vec<NpcDefinition> {
    vec![
        NpcDefinition {
            citizen_id: 1,
            name: "Mira Ashbridge".to_string(),
            gender: Gender::Female,
            age: 42,
            occupation: OccupationType::Innkeeper,
            household_id: 1,
            household_role: HouseholdRole::Head,
            home_location: LocationId(1),
            work_location: LocationId(1),
            base_personality: 12,
            will_teach: Some(caps::COOKING),
            teach_threshold: 30,
            starting_coins: 85.0,
            description: "A stout woman with steady eyes. She runs the inn with practiced efficiency. She watches new arrivals carefully before forming opinions.".to_string(),
            schedule: vec![
                ScheduleSlot { tick_start: 6, activity: NpcActivity::Working, location: LocationId(1) },
                ScheduleSlot { tick_start: 12, activity: NpcActivity::AtMarket, location: LocationId(3) },
                ScheduleSlot { tick_start: 14, activity: NpcActivity::Working, location: LocationId(1) },
                ScheduleSlot { tick_start: 20, activity: NpcActivity::Eating, location: LocationId(1) },
                ScheduleSlot { tick_start: 22, activity: NpcActivity::Sleeping, location: LocationId(1) },
            ],
        },
        NpcDefinition {
            citizen_id: 2,
            name: "Wren Forscythe".to_string(),
            gender: Gender::Male,
            age: 38,
            occupation: OccupationType::Artisan,
            household_id: 2,
            household_role: HouseholdRole::Head,
            home_location: LocationId(2),
            work_location: LocationId(2),
            base_personality: -8,
            will_teach: Some(caps::SMITHING),
            teach_threshold: 45,
            starting_coins: 120.0,
            description: "A broad-shouldered blacksmith with scarred hands and a slow manner of speaking. He seems suspicious of strangers and doesn't offer information freely.".to_string(),
            schedule: vec![
                ScheduleSlot { tick_start: 5, activity: NpcActivity::Working, location: LocationId(2) },
                ScheduleSlot { tick_start: 12, activity: NpcActivity::Eating, location: LocationId(2) },
                ScheduleSlot { tick_start: 13, activity: NpcActivity::Working, location: LocationId(2) },
                ScheduleSlot { tick_start: 18, activity: NpcActivity::AtMarket, location: LocationId(3) },
                ScheduleSlot { tick_start: 21, activity: NpcActivity::Sleeping, location: LocationId(2) },
            ],
        },
        NpcDefinition {
            citizen_id: 3,
            name: "Oswin Cley".to_string(),
            gender: Gender::Male,
            age: 51,
            occupation: OccupationType::Farmer,
            household_id: 3,
            household_role: HouseholdRole::Head,
            home_location: LocationId(4),
            work_location: LocationId(4),
            base_personality: 8,
            will_teach: Some(caps::FARMING),
            teach_threshold: 25,
            starting_coins: 45.0,
            description: "A weathered farmer with sun-creased skin. Talks freely about crops and weather. Has farmed the north fields for twenty years.".to_string(),
            schedule: vec![
                ScheduleSlot { tick_start: 5, activity: NpcActivity::Working, location: LocationId(4) },
                ScheduleSlot { tick_start: 12, activity: NpcActivity::Eating, location: LocationId(4) },
                ScheduleSlot { tick_start: 13, activity: NpcActivity::Working, location: LocationId(5) },
                ScheduleSlot { tick_start: 18, activity: NpcActivity::Socializing, location: LocationId(3) },
                ScheduleSlot { tick_start: 21, activity: NpcActivity::Sleeping, location: LocationId(4) },
            ],
        },
        NpcDefinition {
            citizen_id: 4,
            name: "Sera Cley".to_string(),
            gender: Gender::Female,
            age: 47,
            occupation: OccupationType::Herbalist,
            household_id: 3,
            household_role: HouseholdRole::Spouse,
            home_location: LocationId(6),
            work_location: LocationId(6),
            base_personality: 15,
            will_teach: Some(caps::HERBALISM),
            teach_threshold: 35,
            starting_coins: 30.0,
            description: "Oswin's wife. She moves quietly and smells of dried herbs. She knows which plants cure what and which ones kill. Friendly to those who show genuine curiosity.".to_string(),
            schedule: vec![
                ScheduleSlot { tick_start: 7, activity: NpcActivity::Working, location: LocationId(6) },
                ScheduleSlot { tick_start: 11, activity: NpcActivity::AtMarket, location: LocationId(3) },
                ScheduleSlot { tick_start: 13, activity: NpcActivity::Working, location: LocationId(6) },
                ScheduleSlot { tick_start: 19, activity: NpcActivity::Eating, location: LocationId(4) },
                ScheduleSlot { tick_start: 22, activity: NpcActivity::Sleeping, location: LocationId(4) },
            ],
        },
        NpcDefinition {
            citizen_id: 5,
            name: "Elder Voss".to_string(),
            gender: Gender::Male,
            age: 74,
            occupation: OccupationType::Elder,
            household_id: 4,
            household_role: HouseholdRole::Head,
            home_location: LocationId(10),
            work_location: LocationId(9),
            base_personality: 5,
            will_teach: Some(caps::INSCRIPTION),
            teach_threshold: 60,
            starting_coins: 200.0,
            description: "The settlement elder. Older than most buildings in Thornveil. His memory is long and his words careful. He knows things about the old archive that he doesn't volunteer.".to_string(),
            schedule: vec![
                ScheduleSlot { tick_start: 8, activity: NpcActivity::Resting, location: LocationId(10) },
                ScheduleSlot { tick_start: 10, activity: NpcActivity::Socializing, location: LocationId(9) },
                ScheduleSlot { tick_start: 14, activity: NpcActivity::Resting, location: LocationId(10) },
                ScheduleSlot { tick_start: 17, activity: NpcActivity::Socializing, location: LocationId(7) },
                ScheduleSlot { tick_start: 20, activity: NpcActivity::Eating, location: LocationId(10) },
                ScheduleSlot { tick_start: 21, activity: NpcActivity::Sleeping, location: LocationId(10) },
            ],
        },
        // Additional NPCs for population texture (6–15)
        NpcDefinition {
            citizen_id: 6,
            name: "Tomas Birch".to_string(),
            gender: Gender::Male,
            age: 28,
            occupation: OccupationType::Forester,
            household_id: 5,
            household_role: HouseholdRole::Head,
            home_location: LocationId(11),
            work_location: LocationId(11),
            base_personality: 3,
            will_teach: Some(caps::WOODCUTTING),
            teach_threshold: 20,
            starting_coins: 35.0,
            description: "A young forester who seems more comfortable with trees than people. Teaches woodcutting readily if you're willing to do the work.".to_string(),
            schedule: vec![
                ScheduleSlot { tick_start: 6, activity: NpcActivity::Working, location: LocationId(11) },
                ScheduleSlot { tick_start: 12, activity: NpcActivity::Eating, location: LocationId(11) },
                ScheduleSlot { tick_start: 13, activity: NpcActivity::Working, location: LocationId(11) },
                ScheduleSlot { tick_start: 19, activity: NpcActivity::Socializing, location: LocationId(1) },
                ScheduleSlot { tick_start: 22, activity: NpcActivity::Sleeping, location: LocationId(11) },
            ],
        },
        NpcDefinition {
            citizen_id: 7,
            name: "Delia Croft".to_string(),
            gender: Gender::Female,
            age: 33,
            occupation: OccupationType::Merchant,
            household_id: 6,
            household_role: HouseholdRole::Head,
            home_location: LocationId(3),
            work_location: LocationId(3),
            base_personality: 10,
            will_teach: Some(caps::TRADING),
            teach_threshold: 35,
            starting_coins: 175.0,
            description: "Runs the main market stall. Quick with numbers and words. She notices new faces and potential customers.".to_string(),
            schedule: vec![
                ScheduleSlot { tick_start: 7, activity: NpcActivity::Working, location: LocationId(3) },
                ScheduleSlot { tick_start: 13, activity: NpcActivity::Eating, location: LocationId(3) },
                ScheduleSlot { tick_start: 14, activity: NpcActivity::Working, location: LocationId(3) },
                ScheduleSlot { tick_start: 19, activity: NpcActivity::Socializing, location: LocationId(1) },
                ScheduleSlot { tick_start: 22, activity: NpcActivity::Sleeping, location: LocationId(3) },
            ],
        },
        NpcDefinition {
            citizen_id: 8,
            name: "Harwin Croft".to_string(),
            gender: Gender::Male,
            age: 35,
            occupation: OccupationType::Merchant,
            household_id: 6,
            household_role: HouseholdRole::Spouse,
            home_location: LocationId(3),
            work_location: LocationId(3),
            base_personality: 7,
            will_teach: None,
            teach_threshold: 0,
            starting_coins: 90.0,
            description: "Delia's husband. He handles transport and storage. Less talkative than his wife but reliable.".to_string(),
            schedule: vec![
                ScheduleSlot { tick_start: 7, activity: NpcActivity::Working, location: LocationId(10) },
                ScheduleSlot { tick_start: 12, activity: NpcActivity::Eating, location: LocationId(3) },
                ScheduleSlot { tick_start: 13, activity: NpcActivity::Working, location: LocationId(3) },
                ScheduleSlot { tick_start: 20, activity: NpcActivity::Sleeping, location: LocationId(3) },
            ],
        },
        NpcDefinition {
            citizen_id: 9,
            name: "Pella".to_string(),
            gender: Gender::Female,
            age: 19,
            occupation: OccupationType::Laborer,
            household_id: 1,
            household_role: HouseholdRole::Lodger,
            home_location: LocationId(1),
            work_location: LocationId(5),
            base_personality: 18,
            will_teach: None,
            teach_threshold: 0,
            starting_coins: 8.0,
            description: "A young laborer who lodges at the inn. Works whatever field job is going. Curious and talkative. Knows everyone.".to_string(),
            schedule: vec![
                ScheduleSlot { tick_start: 6, activity: NpcActivity::Working, location: LocationId(5) },
                ScheduleSlot { tick_start: 12, activity: NpcActivity::Eating, location: LocationId(1) },
                ScheduleSlot { tick_start: 13, activity: NpcActivity::Working, location: LocationId(4) },
                ScheduleSlot { tick_start: 18, activity: NpcActivity::Socializing, location: LocationId(1) },
                ScheduleSlot { tick_start: 23, activity: NpcActivity::Sleeping, location: LocationId(1) },
            ],
        },
        NpcDefinition {
            citizen_id: 10,
            name: "Aldous Minner".to_string(),
            gender: Gender::Male,
            age: 62,
            occupation: OccupationType::Miner,
            household_id: 7,
            household_role: HouseholdRole::Head,
            home_location: LocationId(9),
            work_location: LocationId(11),
            base_personality: -5,
            will_teach: None,
            teach_threshold: 0,
            starting_coins: 65.0,
            description: "Old miner with a stiff back and a loud opinion about everything. Mines the stone near the forest edge.".to_string(),
            schedule: vec![
                ScheduleSlot { tick_start: 7, activity: NpcActivity::Working, location: LocationId(11) },
                ScheduleSlot { tick_start: 12, activity: NpcActivity::Eating, location: LocationId(9) },
                ScheduleSlot { tick_start: 13, activity: NpcActivity::Working, location: LocationId(11) },
                ScheduleSlot { tick_start: 17, activity: NpcActivity::Socializing, location: LocationId(7) },
                ScheduleSlot { tick_start: 21, activity: NpcActivity::Sleeping, location: LocationId(9) },
            ],
        },
        // Remaining NPCs: household members, minor roles
        NpcDefinition {
            citizen_id: 11,
            name: "Gwen Minner".to_string(),
            gender: Gender::Female,
            age: 58,
            occupation: OccupationType::Laborer,
            household_id: 7,
            household_role: HouseholdRole::Spouse,
            home_location: LocationId(9),
            work_location: LocationId(5),
            base_personality: 2,
            will_teach: None,
            teach_threshold: 0,
            starting_coins: 20.0,
            description: "Aldous's wife. Works the south field. Quiet but perceptive.".to_string(),
            schedule: vec![
                ScheduleSlot { tick_start: 6, activity: NpcActivity::Working, location: LocationId(5) },
                ScheduleSlot { tick_start: 12, activity: NpcActivity::Eating, location: LocationId(9) },
                ScheduleSlot { tick_start: 13, activity: NpcActivity::Working, location: LocationId(5) },
                ScheduleSlot { tick_start: 20, activity: NpcActivity::Sleeping, location: LocationId(9) },
            ],
        },
        NpcDefinition {
            citizen_id: 12,
            name: "Runn".to_string(),
            gender: Gender::Male,
            age: 16,
            occupation: OccupationType::Laborer,
            household_id: 5,
            household_role: HouseholdRole::Child,
            home_location: LocationId(11),
            work_location: LocationId(11),
            base_personality: 14,
            will_teach: None,
            teach_threshold: 0,
            starting_coins: 3.0,
            description: "Tomas's kid brother. Still learning the trade. Friendly and easily impressed.".to_string(),
            schedule: vec![
                ScheduleSlot { tick_start: 7, activity: NpcActivity::Working, location: LocationId(11) },
                ScheduleSlot { tick_start: 12, activity: NpcActivity::Eating, location: LocationId(11) },
                ScheduleSlot { tick_start: 13, activity: NpcActivity::Socializing, location: LocationId(9) },
                ScheduleSlot { tick_start: 15, activity: NpcActivity::Working, location: LocationId(11) },
                ScheduleSlot { tick_start: 21, activity: NpcActivity::Sleeping, location: LocationId(11) },
            ],
        },
        NpcDefinition {
            citizen_id: 13,
            name: "Corva".to_string(),
            gender: Gender::Female,
            age: 29,
            occupation: OccupationType::Laborer,
            household_id: 8,
            household_role: HouseholdRole::Head,
            home_location: LocationId(1),
            work_location: LocationId(4),
            base_personality: 0,
            will_teach: None,
            teach_threshold: 0,
            starting_coins: 22.0,
            description: "Quiet woman who does fieldwork. She doesn't initiate conversations but responds directly when spoken to.".to_string(),
            schedule: vec![
                ScheduleSlot { tick_start: 6, activity: NpcActivity::Working, location: LocationId(4) },
                ScheduleSlot { tick_start: 12, activity: NpcActivity::Eating, location: LocationId(1) },
                ScheduleSlot { tick_start: 13, activity: NpcActivity::Working, location: LocationId(4) },
                ScheduleSlot { tick_start: 20, activity: NpcActivity::Sleeping, location: LocationId(1) },
            ],
        },
        NpcDefinition {
            citizen_id: 14,
            name: "Bard Tholl".to_string(),
            gender: Gender::Male,
            age: 44,
            occupation: OccupationType::Artisan,
            household_id: 9,
            household_role: HouseholdRole::Head,
            home_location: LocationId(10),
            work_location: LocationId(10),
            base_personality: 5,
            will_teach: None,
            teach_threshold: 0,
            starting_coins: 70.0,
            description: "Makes rope, leather goods, and assorted small items. Practical and businesslike.".to_string(),
            schedule: vec![
                ScheduleSlot { tick_start: 7, activity: NpcActivity::Working, location: LocationId(10) },
                ScheduleSlot { tick_start: 12, activity: NpcActivity::AtMarket, location: LocationId(3) },
                ScheduleSlot { tick_start: 14, activity: NpcActivity::Working, location: LocationId(10) },
                ScheduleSlot { tick_start: 20, activity: NpcActivity::Sleeping, location: LocationId(10) },
            ],
        },
        NpcDefinition {
            citizen_id: 15,
            name: "Nissa Tholl".to_string(),
            gender: Gender::Female,
            age: 41,
            occupation: OccupationType::Laborer,
            household_id: 9,
            household_role: HouseholdRole::Spouse,
            home_location: LocationId(10),
            work_location: LocationId(5),
            base_personality: 9,
            will_teach: None,
            teach_threshold: 0,
            starting_coins: 25.0,
            description: "Bard's wife. Works the south fields in summer, helps at the storage house in winter.".to_string(),
            schedule: vec![
                ScheduleSlot { tick_start: 6, activity: NpcActivity::Working, location: LocationId(5) },
                ScheduleSlot { tick_start: 12, activity: NpcActivity::Eating, location: LocationId(10) },
                ScheduleSlot { tick_start: 13, activity: NpcActivity::Working, location: LocationId(10) },
                ScheduleSlot { tick_start: 20, activity: NpcActivity::Sleeping, location: LocationId(10) },
            ],
        },
    ]
}

// ── Capability Definitions ────────────────────────────────────────────────────

fn capability_defs() -> Vec<CapabilityDefinition> {
    vec![
        CapabilityDefinition {
            id: caps::WOODCUTTING,
            name: "Woodcutting".to_string(),
            description: "Ability to fell trees and split timber. Unlocks working at the Forest Edge.".to_string(),
            practice_resource: Some(ResourceType::Timber),
            unlocks_actions: vec!["Work(Forester) at Forest Edge".to_string()],
        },
        CapabilityDefinition {
            id: caps::SMITHING,
            name: "Smithing".to_string(),
            description: "Ability to work metal at a forge. Unlocks working at Wren's Forge.".to_string(),
            practice_resource: Some(ResourceType::Tools),
            unlocks_actions: vec!["Work(Artisan) at Forge".to_string()],
        },
        CapabilityDefinition {
            id: caps::PERSUASION,
            name: "Persuasion".to_string(),
            description: "Ability to negotiate. Unlocks improved exchange offers.".to_string(),
            practice_resource: None,
            unlocks_actions: vec!["Offer(improved)".to_string()],
        },
        CapabilityDefinition {
            id: caps::HERBALISM,
            name: "Herbalism".to_string(),
            description: "Ability to identify and use herbs. Unlocks herb gathering and healing.".to_string(),
            practice_resource: Some(ResourceType::Herbs),
            unlocks_actions: vec!["Gather(Herbs)".to_string(), "Heal(self)".to_string()],
        },
        CapabilityDefinition {
            id: caps::COOKING,
            name: "Cooking".to_string(),
            description: "Ability to prepare food. Food you prepare provides more satiety.".to_string(),
            practice_resource: Some(ResourceType::Food),
            unlocks_actions: vec!["Cook(Food)".to_string()],
        },
        CapabilityDefinition {
            id: caps::TRACKING,
            name: "Tracking".to_string(),
            description: "Ability to read signs in the forest. Unlocks better foraging.".to_string(),
            practice_resource: None,
            unlocks_actions: vec!["Track(Forest Edge)".to_string()],
        },
        CapabilityDefinition {
            id: caps::INSCRIPTION,
            name: "Inscription".to_string(),
            description: "Ability to record observations in a structured form. Core of the Inscription transformation path. Unlocks Inscribe and StudyArchive actions.".to_string(),
            practice_resource: Some(ResourceType::Ink),
            unlocks_actions: vec!["Inscribe(observation)".to_string(), "StudyArchive".to_string()],
        },
        CapabilityDefinition {
            id: caps::FARMING,
            name: "Farming".to_string(),
            description: "Ability to work the fields productively. Unlocks farming work.".to_string(),
            practice_resource: Some(ResourceType::Food),
            unlocks_actions: vec!["Work(Farmer) at Fields".to_string()],
        },
        CapabilityDefinition {
            id: caps::TRADING,
            name: "Trading".to_string(),
            description: "Ability to evaluate goods and negotiate prices. Unlocks better market terms.".to_string(),
            practice_resource: None,
            unlocks_actions: vec!["Buy/Sell at improved prices".to_string()],
        },
        CapabilityDefinition {
            id: caps::DIAGNOSIS,
            name: "Diagnosis".to_string(),
            description: "Ability to examine agricultural, structural, and ecological symptoms in the field. Core of the Settlement Chronicler path. Unlocks Diagnose and DraftDocument actions.".to_string(),
            practice_resource: None,
            unlocks_actions: vec![
                "Diagnose(Location)".to_string(),
                "DraftDocument(Type)".to_string(),
                "ArbitrateDispute(Document, Consequence)".to_string(),
            ],
        },
    ]
}

// ── Knowledge Nodes ───────────────────────────────────────────────────────────

fn knowledge_nodes() -> Vec<KnowledgeNode> {
    use crate::types::KnowledgeCategory;
    vec![
        KnowledgeNode {
            id: knowledge::ANCIENT_ARCHIVE,
            title: "The Old Archive".to_string(),
            description: "There is a ruined stone structure in Thornveil that once held records. Elder Voss knows its history.".to_string(),
            category: KnowledgeCategory::LocationFact { location: LocationId(8) },
        },
        KnowledgeNode {
            id: knowledge::ELDER_VOSS_SECRET,
            title: "Elder Voss and the Archive".to_string(),
            description: "Elder Voss was the last person to use the archive before it fell into disrepair. He has never spoken of why he stopped.".to_string(),
            category: KnowledgeCategory::TransformationClue { stage: 0 },
        },
        KnowledgeNode {
            id: knowledge::INSCRIPTION_PRIMER,
            title: "The Inscription Primer".to_string(),
            description: "Elder Voss possesses a copy of an inscription primer — a guide to the structured recording of knowledge. He will share it only with someone he trusts.".to_string(),
            category: KnowledgeCategory::TransformationClue { stage: 1 },
        },
        KnowledgeNode {
            id: knowledge::HERB_LOCATIONS,
            title: "Herb Growing Sites".to_string(),
            description: "Sera has shown you where the useful herbs grow in the garden and near the forest edge.".to_string(),
            category: KnowledgeCategory::ResourceFact { resource: ResourceType::Herbs, location: LocationId(6) },
        },
        KnowledgeNode {
            id: knowledge::TIMBER_SOURCES,
            title: "Timber Sources".to_string(),
            description: "Tomas pointed out the best timber stands near the forest edge.".to_string(),
            category: KnowledgeCategory::ResourceFact { resource: ResourceType::Timber, location: LocationId(11) },
        },
        KnowledgeNode {
            id: knowledge::MIRA_HISTORY,
            title: "Mira's Past".to_string(),
            description: "Mira came to Thornveil from elsewhere, twelve years ago. She doesn't talk about where or why.".to_string(),
            category: KnowledgeCategory::NpcFact { about: crate::types::CitizenId(1) },
        },
        KnowledgeNode {
            id: knowledge::WREN_GRUDGE,
            title: "Wren's Grudge".to_string(),
            description: "Wren holds a grudge against someone who left Thornveil years ago. He won't say who. It makes him mistrustful of newcomers.".to_string(),
            category: KnowledgeCategory::NpcFact { about: crate::types::CitizenId(2) },
        },
        KnowledgeNode {
            id: knowledge::SETTLEMENT_FOUNDING,
            title: "The Founding of Thornveil".to_string(),
            description: "Thornveil was founded three generations ago by a group of families fleeing a flood. Elder Voss is descended from the original settlers.".to_string(),
            category: KnowledgeCategory::SettlementHistory,
        },
    ]
}
