/// Godseed Core — Fundamental types
///
/// Newtype IDs, enums, and value types shared across all modules.
/// Derived from CIVITAS-1M types.rs with Godseed extensions.

use serde::{Deserialize, Serialize};

// ── ID Types ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CitizenId(pub u64);

impl CitizenId {
    /// The canonical player citizen ID (reserved)
    pub const PLAYER: CitizenId = CitizenId(0);
}

impl std::fmt::Display for CitizenId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 0 { write!(f, "PLAYER") } else { write!(f, "NPC#{}", self.0) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct HouseholdId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SettlementId(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct LocationId(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CapabilityId(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct KnowledgeNodeId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MilestoneId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SocialGroupId(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ObjectId(pub u32);

// ── Gender ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    Female,
    Male,
}

// ── Occupations ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum OccupationType {
    Unemployed,
    Farmer,
    Forester,
    Miner,
    Artisan,
    Merchant,
    Laborer,
    Innkeeper,
    Herbalist,
    Elder,
}

impl OccupationType {
    pub fn display_name(&self) -> &'static str {
        match self {
            OccupationType::Unemployed => "Unemployed",
            OccupationType::Farmer => "Farmer",
            OccupationType::Forester => "Forester",
            OccupationType::Miner => "Miner",
            OccupationType::Artisan => "Artisan",
            OccupationType::Merchant => "Merchant",
            OccupationType::Laborer => "Laborer",
            OccupationType::Innkeeper => "Innkeeper",
            OccupationType::Herbalist => "Herbalist",
            OccupationType::Elder => "Elder",
        }
    }
}

// ── Resources ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    Food,
    Timber,
    Stone,
    Tools,
    Luxury,
    Herbs,
    Ink,
    Parchment,
}

impl ResourceType {
    pub const ALL: [ResourceType; 8] = [
        ResourceType::Food, ResourceType::Timber, ResourceType::Stone,
        ResourceType::Tools, ResourceType::Luxury, ResourceType::Herbs,
        ResourceType::Ink, ResourceType::Parchment,
    ];

    pub fn display_name(&self) -> &'static str {
        match self {
            ResourceType::Food => "Food",
            ResourceType::Timber => "Timber",
            ResourceType::Stone => "Stone",
            ResourceType::Tools => "Tools",
            ResourceType::Luxury => "Luxury Goods",
            ResourceType::Herbs => "Herbs",
            ResourceType::Ink => "Ink",
            ResourceType::Parchment => "Parchment",
        }
    }
}

// ── Biomes ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Biome {
    Plains,
    Forest,
    Hills,
    Water,
    Settlement,
}

// ── Migration / Mobility ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationStatus {
    Settled,
    InTransit {
        origin: SettlementId,
        destination: SettlementId,
        ticks_remaining: u16,
        total_ticks: u16,
    },
}

// ── Household ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdRole {
    Head,
    Spouse,
    Child,
    Dependent,
    Lodger,
}

// ── NPC Activities ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NpcActivity {
    Sleeping,
    Working,
    AtMarket,
    Socializing,
    Eating,
    Resting,
    Patrolling,
    Idle,
}

impl NpcActivity {
    pub fn display(&self) -> &'static str {
        match self {
            NpcActivity::Sleeping => "sleeping",
            NpcActivity::Working => "working",
            NpcActivity::AtMarket => "at the market",
            NpcActivity::Socializing => "socializing",
            NpcActivity::Eating => "eating",
            NpcActivity::Resting => "resting",
            NpcActivity::Patrolling => "patrolling",
            NpcActivity::Idle => "idle",
        }
    }
}

// ── NPC Goals ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NpcGoal {
    SatisfyHunger,
    EarnMoney { target_amount: u32 },
    SocializeWith { target: CitizenId },
    CompleteWork,
    Rest,
    GoHome,
    GatherResource { resource: ResourceType },
}

// ── Decision Trace ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReasonCode {
    NaturalBirth,
    OldAgeSenescence,
    StarvationDeath,
    JobOpportunityFound,
    JobLaidOff,
    HouseholdFormed,
    HouseholdDissolved,
    MigrationBetterWages,
    MigrationFleeingHunger,
    InitialSpawn,
    PlayerArrival,
    CapabilityAcquired,
    RelationshipChanged,
    TransformationProgress,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionTrace {
    pub reason: ReasonCode,
    pub tick: u64,
    pub primary_metric: f32,
    pub secondary_metric: f32,
    pub context_id: u32,
}

impl DecisionTrace {
    pub fn new(reason: ReasonCode, tick: u64, primary: f32, secondary: f32, ctx: u32) -> Self {
        Self { reason, tick, primary_metric: primary, secondary_metric: secondary, context_id: ctx }
    }
}

// ── Capability Level ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CapabilityLevel(pub u8); // 0 = none, 1 = novice, 2 = skilled, 3 = expert

impl CapabilityLevel {
    pub const NONE: Self = CapabilityLevel(0);
    pub const NOVICE: Self = CapabilityLevel(1);
    pub const SKILLED: Self = CapabilityLevel(2);
    pub const EXPERT: Self = CapabilityLevel(3);

    pub fn display(&self) -> &'static str {
        match self.0 {
            1 => "Novice",
            2 => "Skilled",
            3 => "Expert",
            _ => "None",
        }
    }
}

// ── Transformation ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransformationPath {
    None,
    Inscription,
}

// ── Knowledge Node Types ──────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KnowledgeCategory {
    NpcFact { about: CitizenId },
    LocationFact { location: LocationId },
    ResourceFact { resource: ResourceType, location: LocationId },
    CapabilityPrerequisite { capability: CapabilityId },
    TransformationClue { stage: u8 },
    SettlementHistory,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeNode {
    pub id: KnowledgeNodeId,
    pub title: String,
    pub description: String,
    pub category: KnowledgeCategory,
}

// ── NPC Memory Event ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryEventType {
    MetPlayer,
    PlayerHelped,
    PlayerHarmed,
    PlayerStole,
    PlayerWorked,
    PlayerGaveGift,
    ObservedEvent,
    HeardGossip { from: CitizenId },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryEvent {
    pub tick: u64,
    pub event_type: MemoryEventType,
    pub subject: CitizenId,   // who the event was about
    pub impact: i8,            // -5 to +5 disposition impact
    pub description: String,
}

// ── NPC Schedule ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduleSlot {
    pub tick_start: u32, // tick within the day (0–23 for hourly)
    pub activity: NpcActivity,
    pub location: LocationId,
}

// ── Reputation ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReputationRecord {
    pub value: i16, // -100..+100
    pub known_by_count: u32,
}


// ── Player Actions ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlayerAction {
    Move { to: LocationId },
    Look,
    Inspect { target: CitizenId },
    InspectObject { object: ObjectId },
    Talk { npc: CitizenId, topic: TalkTopic },
    Offer { npc: CitizenId, exchange: Exchange },
    Buy { resource: ResourceType, quantity: u32 },
    Sell { resource: ResourceType, quantity: u32 },
    PickUp { object: ObjectId },
    Drop { object: ObjectId },
    Work { occupation: OccupationType },
    Practice { capability: CapabilityId },
    LearnFrom { npc: CitizenId, capability: CapabilityId },
    Inscribe { observation: String },
    StudyArchive,
    Wait { ticks: u32 },
    Sleep,
    Save { path: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TalkTopic {
    Greeting,
    AskAbout { subject: CitizenId },
    AskAboutLocation { location: LocationId },
    AskAboutWork,
    RequestWork,
    AskAboutTransformation,
    ShareKnowledge { node: KnowledgeNodeId },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Exchange {
    pub offer_resource: Option<(ResourceType, u32)>,
    pub offer_coins: f64,
    pub request_resource: Option<(ResourceType, u32)>,
    pub request_capability_teaching: Option<CapabilityId>,
    pub request_coins: f64,
}

// ── Action Result ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub tick: u64,
    pub success: bool,
    pub message: String,
    pub side_effects: Vec<SideEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SideEffect {
    RelationshipChanged { npc: CitizenId, delta: i16 },
    ResourceGained { resource: ResourceType, quantity: u32 },
    ResourceLost { resource: ResourceType, quantity: u32 },
    CoinsGained(f64),
    CoinsLost(f64),
    CapabilityGained { capability: CapabilityId, level: CapabilityLevel },
    KnowledgeGained { node: KnowledgeNodeId },
    TransformationProgress { stage: u8, progress: u16 },
    ReputationChanged { group: SocialGroupId, delta: i16 },
    PlayerMoved { to: LocationId },
}

// ── Sim Clock ─────────────────────────────────────────────────────────────────

#[derive(bevy_ecs::system::Resource, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimClock {
    pub tick: u64,
}

impl SimClock {
    pub fn new() -> Self { Self { tick: 0 } }
    pub fn day(&self) -> u64 { self.tick / 24 }
    pub fn hour(&self) -> u64 { self.tick % 24 }
    pub fn month(&self) -> u64 { self.day() / 30 }
}

impl Default for SimClock {
    fn default() -> Self { Self::new() }
}

// ── Map Cell ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CellType {
    Road,
    Building { location_id: LocationId },
    Field,
    Forest,
    Water,
    Open,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MapCell {
    pub x: u8,
    pub y: u8,
    pub cell_type: CellType,
    pub passable: bool,
}

// ── NPC Fact (what an NPC knows about another entity) ─────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NpcFact {
    pub last_seen_tick: u64,
    pub known_location: Option<LocationId>,
    pub reputation_assessment: i8, // -5 to +5
}
