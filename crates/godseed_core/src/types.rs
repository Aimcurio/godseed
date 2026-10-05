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
        if self.0 == 0 {
            write!(f, "PLAYER")
        } else {
            write!(f, "NPC#{}", self.0)
        }
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
        ResourceType::Food,
        ResourceType::Timber,
        ResourceType::Stone,
        ResourceType::Tools,
        ResourceType::Luxury,
        ResourceType::Herbs,
        ResourceType::Ink,
        ResourceType::Parchment,
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
        Self {
            reason,
            tick,
            primary_metric: primary,
            secondary_metric: secondary,
            context_id: ctx,
        }
    }
}

// ── Capability Level ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CapabilityLevel(pub u8); // 0 = none, 1 = novice, 2 = skilled, 3 = expert

impl CapabilityLevel {
    pub const NONE: Self = CapabilityLevel(0);
    pub const NOVICE: Self = CapabilityLevel(1);
    pub const SKILLED: Self = CapabilityLevel(2);
    pub const JOURNEYMAN: Self = CapabilityLevel(2);
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
    NpcFact {
        about: CitizenId,
    },
    LocationFact {
        location: LocationId,
    },
    ResourceFact {
        resource: ResourceType,
        location: LocationId,
    },
    CapabilityPrerequisite {
        capability: CapabilityId,
    },
    TransformationClue {
        stage: u8,
    },
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
    pub subject: CitizenId, // who the event was about
    pub impact: i8,         // -5 to +5 disposition impact
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
    Move {
        to: LocationId,
    },
    Look,
    Inspect {
        target: CitizenId,
    },
    InspectObject {
        object: ObjectId,
    },
    Talk {
        npc: CitizenId,
        topic: TalkTopic,
    },
    Offer {
        npc: CitizenId,
        exchange: Exchange,
    },
    Buy {
        resource: ResourceType,
        quantity: u32,
    },
    Sell {
        resource: ResourceType,
        quantity: u32,
    },
    PickUp {
        object: ObjectId,
    },
    Drop {
        object: ObjectId,
    },
    Work {
        occupation: OccupationType,
    },
    Practice {
        capability: CapabilityId,
    },
    LearnFrom {
        npc: CitizenId,
        capability: CapabilityId,
    },
    Inscribe {
        observation: String,
    },
    StudyArchive,
    Wait {
        ticks: u32,
    },
    Sleep,
    Save {
        path: String,
    },
    HelpWithFelling {
        npc: CitizenId,
    },
    Diagnose {
        location: LocationId,
    },
    DraftDocument {
        doc_type: DocumentType,
    },
    ArbitrateDispute {
        document_id: u32,
        consequence_id: u32,
    },
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
    RelationshipChanged {
        npc: CitizenId,
        delta: i16,
    },
    ResourceGained {
        resource: ResourceType,
        quantity: u32,
    },
    ResourceLost {
        resource: ResourceType,
        quantity: u32,
    },
    CoinsGained(f64),
    CoinsLost(f64),
    CapabilityGained {
        capability: CapabilityId,
        level: CapabilityLevel,
    },
    KnowledgeGained {
        node: KnowledgeNodeId,
    },
    TransformationProgress {
        stage: u8,
        progress: u16,
    },
    ReputationChanged {
        group: SocialGroupId,
        delta: i16,
    },
    PlayerMoved {
        to: LocationId,
    },
    DocumentCreated {
        id: u32,
    },
    DisputeArbitrated {
        consequence_id: u32,
        document_id: u32,
    },
}

// ── Sim Clock ─────────────────────────────────────────────────────────────────

#[derive(bevy_ecs::system::Resource, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimClock {
    pub tick: u64,
}

impl SimClock {
    pub fn new() -> Self {
        Self { tick: 0 }
    }
    pub fn day(&self) -> u64 {
        self.tick / 24
    }
    pub fn hour(&self) -> u64 {
        self.tick % 24
    }
    pub fn month(&self) -> u64 {
        self.day() / 30
    }
}

impl Default for SimClock {
    fn default() -> Self {
        Self::new()
    }
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

// ── VS2 Causal & Meaning Primitives ──────────────────────────────────────────

/// Lightweight Causal Pointer for explainable causality (NC-44)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CausalPointer {
    pub root_event_id: u64,
    pub parent_event_id: u64,
    pub sequence_step: u8,
}

/// Derived behavioral mode for qualitative relational divergence (AC-201)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BehavioralMode {
    DevotedAlly,
    AffectionateRefusal,
    GrudgingDebtor,
    WaryConsultant,
    HardenedEnemy,
}

/// Knowledge domain classification (AC-204, Architecture Final Sec 8)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KnowledgeDomain {
    ObservationInsight, // e.g. Crop Blight Signs, Timber Stress, Herb Habitats
    SecretTruth,        // e.g. Voss's Exiled Son, Delia's Debt, Founding Flood
    DocumentedRecord,   // e.g. Ancient Land Charter, Signed Debt Note
}

/// Rich knowledge definition with social fallout modes (Architecture Final Sec 8)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeDefinition {
    pub id: u16,
    pub domain: KnowledgeDomain,
    pub title: String,
    pub description: String,
    pub social_fallout_mode: Option<BehavioralMode>,
}

/// Compact 4-byte Triad Relational Bond (Sentiment, Trust, Obligation)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalBond {
    pub sentiment: i8,   // -100 to +100 (Emotional Warmth vs. Hostility)
    pub trust: i8,       // -100 to +100 (Reliability vs. Suspicion)
    pub obligation: i16, // -1000 to +1000 (Positive = owes target, Negative = target owes)
}

impl RelationalBond {
    pub fn new(sentiment: i8, trust: i8, obligation: i16) -> Self {
        Self {
            sentiment: sentiment.clamp(-100, 100),
            trust: trust.clamp(-100, 100),
            obligation: obligation.clamp(-1000, 1000),
        }
    }

    pub fn mode(&self) -> BehavioralMode {
        if self.obligation >= 50 && self.sentiment < -20 {
            BehavioralMode::GrudgingDebtor
        } else if self.sentiment > 30 && self.trust < -20 {
            BehavioralMode::AffectionateRefusal
        } else if self.sentiment.abs() <= 20 && self.trust >= 50 {
            BehavioralMode::WaryConsultant
        } else if self.sentiment > 40 && self.trust > 40 {
            BehavioralMode::DevotedAlly
        } else if self.sentiment < -40 && self.trust < -30 {
            BehavioralMode::HardenedEnemy
        } else {
            BehavioralMode::WaryConsultant
        }
    }
}

impl Default for RelationalBond {
    fn default() -> Self {
        Self {
            sentiment: 0,
            trust: 0,
            obligation: 0,
        }
    }
}

/// Memory tags for episodic turning points and events (AC-202)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryTag {
    HelpedWithFelling,
    SavedLife,
    Betrayal,
    ContractSigned,
    HeardGossipAbout(CitizenId),
    CasualInteraction,
    ObservationShared,
}

/// Bounded episodic record (AC-202)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpisodicRecord {
    pub id: u64,
    pub tick: u64,
    pub actor: CitizenId,
    pub target: Option<CitizenId>,
    pub tag: MemoryTag,
    pub delta_sentiment: i8,
    pub delta_trust: i8,
    pub delta_obligation: i16,
    pub is_permanent: bool,
    pub narrative_token: u16,
    pub causal: Option<CausalPointer>,
}

/// Consequence stage progression (AC-206, AC-207)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsequenceStage {
    Active,
    Escalated,
    Matured,
    Resolved,
}

/// Typed consequence payloads for autonomous situations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsequenceType {
    FraternalLaborStrain {
        elder: CitizenId,
        junior: CitizenId,
        target_workplace: LocationId,
    },
    CropBlightDispute {
        farmer_a: CitizenId,
        farmer_b: CitizenId,
        location: LocationId,
    },
    DebtDispute {
        creditor: CitizenId,
        debtor: CitizenId,
        amount: u32,
    },
}

/// Typed inscribed documents created through scholar documentary authority (AC-205, Architecture Sec 12)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentType {
    DebtReliefCharter {
        creditor: CitizenId,
        debtor: CitizenId,
        terms: u32,
    },
    HarvestDiagnosisReport {
        location: LocationId,
        finding: u16,
    },
    FoundingArchiveTranslation {
        secret_id: u16,
    },
}

impl DocumentType {
    pub fn title(&self) -> String {
        match self {
            DocumentType::DebtReliefCharter {
                creditor,
                debtor,
                terms,
            } => {
                format!(
                    "Charter of Debt Relief: Citizen {} to Citizen {} ({} coins)",
                    creditor.0, debtor.0, terms
                )
            }
            DocumentType::HarvestDiagnosisReport { location, finding } => {
                format!(
                    "Official Harvest & Soil Diagnosis for Location #{} (Finding #{})",
                    location.0, finding
                )
            }
            DocumentType::FoundingArchiveTranslation { secret_id } => {
                format!("Ancient Inscription Translation: Record #{}", secret_id)
            }
        }
    }
}

/// Physical or registered inscribed document with documentary binding authority
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InscribedDocument {
    pub id: u32,
    pub doc_type: DocumentType,
    pub drafter: CitizenId,
    pub signers: Vec<CitizenId>,
    pub binding_tick: u64,
    pub related_consequence_id: Option<u32>,
}

/// Composable trigger conditions (supporting SimLab 3-trigger or simple latency)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriggerCondition {
    TimeElapsed { duration_ticks: u64 },
    Compound(Vec<TriggerCondition>),
}

/// Authoritative pending consequence tracked across time and absence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingConsequence {
    pub id: u32,
    pub causal_root: u64,
    pub stage: ConsequenceStage,
    pub trigger: TriggerCondition,
    pub consequence_type: ConsequenceType,
    pub created_tick: u64,
}
