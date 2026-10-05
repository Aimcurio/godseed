# GODSEED VERTICAL SLICE 2: FROZEN TECHNICAL ARCHITECTURE
## Authoritative Systems Architecture for Meaning, Attachment & Consequence

**Document Identifier:** `GODSEED-VS2-ARCH-FINAL`  
**Governing Product Authority:** [`GODSEED_VS2_PRODUCT_CONTRACT_REVISED.md`](file:///C:/Users/15103/.gemini/antigravity/scratch/godseed/GODSEED_VS2_PRODUCT_CONTRACT_REVISED.md) (`E9CD4A2A79AFCA4970D301A41619B9639A761A82630194296CC18EEE2CE72296`)  
**Product Review Reference:** [`GODSEED_VS2_PRODUCT_REVIEW.md`](file:///C:/Users/15103/.gemini/antigravity/scratch/godseed/GODSEED_VS2_PRODUCT_REVIEW.md) (`26569193B75E15A912E7A4EC79901FAEC07295FBD879AF6CF16677C11A61348B`)  
**Final Qualification Reference:** [`GODSEED_VS2_FINAL_ARCHITECTURE_REVIEW.md`](file:///C:/Users/15103/.gemini/antigravity/scratch/godseed/GODSEED_VS2_FINAL_ARCHITECTURE_REVIEW.md) (`8BD51ED0087CF95C92778F820B0F471E1240155ECC1A8ADCDE52027AA5E57AAE`)  
**Predecessor Architecture SHA-256:** `43B18F11E22B72083C23D0F11264C6D1FB8F762472C1413826F1E8B0A375AEAD`  
**Predecessor Optimization SHA-256:** `2AD52AB4B9860519FA8ACBB13A59F500869B4BC9EF24614608CC5536B06447AE`  
**Baseline Candidate:** `391d2937379d775d12da2425001ea2fc88b733b4` (VS1 Candidate)  
**Authority Disposition:** `GODSEED_VS2_IMPLEMENTATION_AUTHORIZED`  
**Status:** **FROZEN AUTHORITY** (Implementation in Progress)

---

## 1. Architecture Purpose & Core Invariants

The Godseed Vertical Slice 2 technical architecture defines the minimal, bounded computational machinery required to execute the approved product contract: transforming a functioning settlement simulation into a world where relationships, discoveries, absences, and consequences carry genuine meaning for the player.

### The Universal Player Experience Loop:
$$\text{Observe \& Connect} \longrightarrow \text{Encounter Tension} \longrightarrow \text{Intervene / Refrain} \longrightarrow \text{Live with Ripple} \longrightarrow \text{Experience Absent World} \longrightarrow \text{Reap Delayed Consequence}$$

### Core Non-Negotiable Invariants:
1. **Determinism:** Bit-identical simulation state from any identical seed and input stream. Zero float nondeterminism; zero system clock dependencies.
2. **State Boundedness ($O(1)$ RAM):** Strict $O(1)$ memory allocation per NPC over infinite simulation ticks. Zero unbounded history logs or memory leaks.
3. **Auditable Causal Lineage:** Every delayed consequence, relationship shift, and return salutation is causally traceable back to a specific prior root event (`CausalPointer`).
4. **Cognitive Integrity (No Omniscience):** System causal truth is strictly partitioned from NPC knowledge and belief. NPCs react solely to observed or communicated facts.
5. **Preservation of Foundation:** 100% backward compatibility with VS1 baseline systems, maintaining $\ge 250,000$ ticks/sec headless simulation throughput.

---

## 2. Product Boundaries & Hard Constraints

From `GODSEED_VS2_PRODUCT_CONTRACT_REVISED.md`, the architecture operates under strict non-negotiable boundaries:
- **Spatial Scope:** Exactly 11 spatial nodes within Thornveil Settlement. No outer regions or multi-settlement maps.
- **Population Scope:** Exactly the 15 authored citizens from VS1. Zero procedural NPCs.
- **Inhabitant Tiers:** 4 Anchor NPCs, 5 Focal NPCs, 6 Texture NPCs. Component interfaces are uniform; depth varies by authored definitions.
- **Zero Generative AI Runtime:** Zero non-deterministic LLM calls, zero floating-point embeddings, zero vector databases. Pure deterministic integer ECS.
- **Zero Combat Subsystems:** Zero weapons, hit-point damage, or military combat mechanics.
- **Save Integrity:** Strict persistence round-tripping with schema evolution from `GODSEED1` to `GODSEED2`.

---

## 3. VS1 Baseline & System Evolution

```text
┌────────────────────────────────────────────────────────────────────────┐
│                        VS1 → VS2 SYSTEM DELTAS                         │
├──────────────────────────┬──────────────┬──────────────────────────────┤
│ System                   │ Status       │ Architectural Delta          │
├──────────────────────────┼──────────────┼──────────────────────────────┤
│ PhysiologySystem         │ UNCHANGED    │ Satiety/health decay         │
│ ProductionSystem         │ UNCHANGED    │ Labor → commodity output     │
│ MarketPriceSystem        │ UNCHANGED    │ Elastic supply/demand pricing│
│ NpcRoutineSystem         │ EXTENDED     │ Supports dynamic work slots  │
│ RelationshipSystem       │ REPLACED     │ Triad bond (Sentiment/Trust/ │
│                          │              │ Obligation) + Behavioral Mode│
│ NpcMemorySystem          │ REPLACED     │ Bounded dual-stream records  │
│                          │              │ (12 transient + 6 anchors)   │
│ GossipSystem             │ REPLACED     │ Asymmetric corroborating     │
│                          │              │ narrative gossip engine      │
│ KnowledgeSystem          │ EXTENDED     │ Asymmetric EpistemicState    │
│ PlayerActionSystem       │ EXTENDED     │ Adds Diagnose & InscribeDoc  │
│ PendingConsequenceSystem │ NEW          │ Autonomous continuation &    │
│                          │              │ delayed consequence pipeline │
│ ReturnDigestSystem       │ NEW          │ Absence return salutations   │
└──────────────────────────┴──────────────┴──────────────────────────────┘
```

---

## 4. System Architecture & Multi-Rate Schedules

```text
┌─────────────────────────────────────────────────────────────────────────┐
│                           TERMINAL UI LAYER                             │
│       (crossterm; renders viewports, dialogue, and return digests)       │
└────────────────────────────────────┬────────────────────────────────────┘
                                     │  PlayerAction queue
┌────────────────────────────────────▼────────────────────────────────────┐
│                    GODSEED SIMULATION ENGINE (Bevy ECS)                 │
│                                                                         │
│  ENTITIES (1 Player + 15 Inhabitants)                                  │
│  ┌─────────────────────────┐         ┌───────────────────────────────┐ │
│  │     COMMON CITIZEN      │         │         PLAYER ENTITY         │ │
│  ├─────────────────────────┤         ├───────────────────────────────┤ │
│  │ CitizenMeta             │         │ CitizenMeta                   │ │
│  │ Demographics            │         │ PhysicalNeeds                 │ │
│  │ PhysicalNeeds           │         │ Inventory                     │ │
│  │ PersonalFinances        │         │ CapabilitySet                 │ │
│  │ Inventory               │         │ EpistemicState                │ │
│  │ SettlementRef           │         │ TransformationState           │ │
│  │ HouseholdRef            │         │ PlayerInputBuffer             │ │
│  │ NpcSchedule             │         │ PlayerMarker                  │ │
│  │ EpisodicMemory (NEW)    │         └───────────────────────────────┘ │
│  │ EpistemicState (NEW)    │                                           │
│  │ RelationalLedger (NEW)  │                                           │
│  │ NpcGoals                │                                           │
│  └─────────────────────────┘                                           │
│                                                                         │
│  RESOURCES                                                              │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │ SimClock              | WorldMap          | SettlementDirectory  │  │
│  │ HouseholdDirectory    | EventRing         | ContentDefinitions   │  │
│  │ PendingConsequenceRegistry (NEW)          | ReturnDigestLog (NEW)│  │
│  │ TelemetryLog                              | NextCausalId (NEW)   │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  SCHEDULES (Multirate Execution)                                        │
│  ├── Hourly Tick:                                                       │
│  │   1. NpcRoutineSystem (moves NPCs per schedule)                      │
│  │   2. PlayerActionSystem (drains input, resolves actions)             │
│  │   3. ProductionSystem (labor produces goods)                         │
│  │   4. PhysiologySystem (metabolic decay)                              │
│  │   5. NpcGoalSystem (evaluates active goals)                          │
│  │                                                                      │
│  ├── Daily Tick (every 24 ticks):                                       │
│  │   1. EpisodicMemoryConsolidationSystem (prunes/compacts memories)    │
│  │   2. PendingConsequenceProgressionSystem (evaluates active triggers) │
│  │   3. DemographicsAgingSystem                                         │
│  │   4. TelemetryEmissionSystem                                         │
│  │                                                                      │
│  ├── Weekly Tick (every 7 days / 168 ticks):                            │
│  │   1. MarketPriceSystem (recalculates supply/demand curves)           │
│  │   2. NarrativeGossipSystem (asymmetric story exchange)               │
│  │   3. HouseholdConsumptionSystem                                      │
│  │   4. RelationalPassiveNormalizationSystem (exponential baseline drift)│
│  │                                                                      │
│  └── Monthly Tick (every 30 days / 720 ticks):                          │
│      1. TransformationCheckSystem (evaluates Scholar Stage 2)           │
│      2. MacroAbsenceResolver (instantaneous multi-week fast-skip)       │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 5. Authoritative State Ownership & Memory Discipline

| Component / Resource | Owner | Auth / Deriv | Persisted | Memory Bounds | Why Required / Product Mapping |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `RelationalLedger` | NPC Entity | Auth | YES | Max 15 bonds (60 B) | AC-201: Qualitative relational divergence |
| `EpisodicMemory` | NPC Entity | Auth | YES | Max 18 records (540 B)| AC-202: Permanent turning-point recall |
| `EpistemicState` | NPC + Player | Auth | YES | Max 32 nodes (256 B) | AC-204: Asymmetric knowledge leverage |
| `PendingConsequenceRegistry` | Resource | Auth | YES | Max 16 consequences (1 KB)| AC-206, AC-207: Delayed consequence & absence |
| `ReturnDigestLog` | Resource | Auth | YES | Max 8 entries (512 B)| AC-207, AC-208: Absence return experience |
| `DocumentRegistry` | Resource | Auth | YES | Max 32 items (2 KB) | AC-205: Inscription & documentary authority |
| `EventRing` | Resource | Auth | YES | Max 1,024 events | Causal auditing & deterministic replay |
| `TelemetryLog` | Resource | Deriv | NO | Max 50,000 events | Session diagnostics & kill-test validation |

---

## 6. Relationship Architecture: The Triad Bond

### Minimal Dimensional Representation
The relationship between an inhabitant and another entity is represented by a compact 4-byte integer struct:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalBond {
    pub sentiment: i8,    // -100 to +100 (Emotional Warmth vs. Hostility)
    pub trust: i8,        // -100 to +100 (Perceived Reliability vs. Suspicion)
    pub obligation: i16,  // -1000 to +1000 (Positive = NPC owes target; Negative = target owes NPC)
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalLedger {
    /// Target CitizenId.0 -> RelationalBond
    pub bonds: HashMap<u64, RelationalBond>,
}
```

### Derived Behavioral Modes
To prevent complex scattered branching across game systems, systems query a clean derived `BehavioralMode`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BehavioralMode {
    DevotedAlly,        // High Sentiment (>40), High Trust (>40)
    AffectionateRefusal,// High Sentiment (>30), Low Trust (<-20) -> Loves you, but refuses tool/secrets
    GrudgingDebtor,     // Hostile Sentiment (<-20), High Obligation (>50) -> Hates you, but must help
    WaryConsultant,     // Neutral Sentiment (-20..20), High Trust (>50) -> Professional scholarly peer
    HardenedEnemy,      // Hostile Sentiment (<-40), Low Trust (<-30), No Debt
}

impl RelationalBond {
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
            BehavioralMode::WaryConsultant // Default baseline
        }
    }
}
```

### Relational Normalization (Calibration Design)
`relational_passive_normalization_system` executes on the weekly tick ($\Delta T = 168 \text{ ticks}$), applying discrete half-life decay toward baseline:
- **Baseline:** $S=0, T=0, O=0$ (unless an active permanent memory anchor defines a non-zero residual anchor baseline).
- **Decay Formula:** Integer bit-shift decay:
  $$\text{sentiment} \leftarrow \text{sentiment} - \text{sign}(\text{sentiment}) \cdot \max\left(1, \frac{|\text{sentiment}|}{8}\right)$$
- `[CALIBRATION]` Half-life $T_{1/2} = 14 \text{ days}$ ($336 \text{ ticks}$).

---

## 7. Memory Architecture: Dual-Stream Bounded Records

### Dual-Stream Memory Isolation
To prevent abstract emotional impressions from displacing concrete factual evidence (`NC-72`):
- **Stance Stream:** `RelationalBond` $(S, T, O)$ maintains the continuous baseline.
- **Evidence Stream:** `EpisodicMemory` maintains concrete, historical turning points.

```rust
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
    pub is_permanent: bool,   // True for turning points: life-saving, severe betrayal, contract
    pub narrative_token: u16, // Maps to authored dialogue template
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpisodicMemory {
    /// Rolling window of transient everyday events (max 12)
    pub transient: VecDeque<EpisodicRecord>,
    /// Permanent turning-point memories (max 6)
    pub anchors: Vec<EpisodicRecord>,
}
```

### Two-Stage Relevance-Dominant Memory Retrieval
Gleaned from SimLab Phase 4 empirical findings (`EXP-AB-01-COMP`):
1. **Stage 1 (Domain Filtering):** Only candidate memories matching the active conversation topic, trade context, or requested assistance are admitted:
   $$\text{CandidateSet} = \{ m \in \text{EpisodicMemory} \mid m.\text{tag} \in \text{Topic}.\text{compatible\_tags}() \}$$
2. **Stage 2 (Relevance-Dominant Ranking):**
   $$\text{Score}(m) = W_{\text{rel}} \cdot \text{Relevance}(m) + W_{\text{rec}} \cdot \text{Recency}(m) + \text{AnchorBonus}$$
   - `[INITIAL_CALIBRATION]` $W_{\text{rel}} = 0.7$, $W_{\text{rec}} = 0.3$.
   - $\text{Relevance}(m) = 1.0$ if $m.\text{actor} == \text{requester}$, and $0.5$ if $m.\text{target} == \text{requester}$.
   - $\text{Recency}(m) = \lambda^{\Delta T}$ with half-life $t_{1/2} = 14 \text{ days}$.
   - Permanent anchors receive an `AnchorBonus` ($+0.25$), but topic relevance is always mandatory.

---

## 8. Knowledge Architecture: Epistemic State & Asymmetric Gossip

### The Tripartite Knowledge Model
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KnowledgeDomain {
    ObservationInsight, // e.g. Crop Blight Signs, Timber Stress, Herb Habitats
    SecretTruth,        // e.g. Voss's Exiled Son, Delia's Debt, Founding Flood
    DocumentedRecord,   // e.g. Ancient Land Charter, Signed Debt Note
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeDefinition {
    pub id: u16,
    pub domain: KnowledgeDomain,
    pub title: String,
    pub description: String,
    pub social_fallout_mode: Option<BehavioralMode>,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EpistemicState {
    /// KnowledgeId -> (AcquiredTick, CorroborationCount)
    pub known: HashMap<u16, (u64, u8)>,
}
```

### Asymmetric Corroborating Gossip System
On the weekly schedule, socializing NPCs exchange knowledge and anchor stories. Rather than a naive ban on repeating known information:
1. **Novel Transfer:** If Listener lacks the knowledge node, it is inserted with corroboration count = 1.
2. **Corroborating Transfer:** If Listener already possesses the knowledge node but with corroboration $< 3$, corroboration increments and confidence strengthens (e.g. hearsay becomes certainty).
3. **No-Op Suppression:** If Listener already has maximum corroboration ($\ge 3$) and no relational update occurs, the transmission is skipped with zero ECS churn.

---

## 9. Causal Lineage & Cognitive Firewall

### The Cognitive Firewall
The architecture enforces strict separation between four causal concepts:
1. **System Truth:** The actual simulation event chain recorded in `EventRing`.
2. **Agent Knowledge:** Factual events registered in an NPC's `EpistemicState` or `EpisodicMemory`.
3. **Agent Belief:** Interpretations or attributions held by an NPC (may be mistaken or biased).
4. **Player-Facing Explanation:** Dialogue and journal entries legitimately revealable to the player.

**Rule:** Simulation truth never leaks into NPC behavior. An NPC can only react to what they observed or were told.

### The Lightweight Causal Pointer
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CausalPointer {
    pub root_event_id: u64,     // The initial player action / decision
    pub parent_event_id: u64,   // The immediate preceding cause
    pub sequence_step: u8,      // Step count (1 = direct, 2 = secondary, etc.)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimEvent {
    pub event_id: u64,
    pub tick: u64,
    pub actor: CitizenId,
    pub causal: Option<CausalPointer>,
    pub payload: EventPayload,
}
```

---

## 10. Consequence Architecture: Pending Consequences & Absence

### Authoritative Pending Consequence Registry
Replaces the abstract `SocialVectorRegistry`. Represents active, unresolved situations in Thornveil:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsequenceStage {
    Active,     // Initial ripple occurring; early subtle dialogue cues
    Escalated,  // Tension building; secondary NPCs affected
    Matured,    // Consequence finalized; structural change committed
    Resolved,   // Peacefully resolved or arbitrated by player intervention
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingConsequence {
    pub id: u32,
    pub causal_root: u64,
    pub stage: ConsequenceStage,
    pub target_a: CitizenId,
    pub target_b: CitizenId,
    pub trigger: TriggerCondition,
    pub consequence_type: ConsequenceType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerCondition {
    TimeElapsed { duration_ticks: u64 },
    SettlementMetric { check: MetricCheck },
    RelationalThreshold { citizen: CitizenId, target: CitizenId, mode: BehavioralMode },
    Compound(Vec<TriggerCondition>), // Multi-trigger convergence (SimLab pattern)
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize, Default)]
pub struct PendingConsequenceRegistry {
    pub consequences: Vec<PendingConsequence>,
}
```

### Execution & The Return Experience
1. **Daily Progression:** Evaluates trigger conditions. When met, consequence advances to `Escalated` or `Matured`, updating schedules, inventories, or bonds.
2. **Absence Fast-Forward:** When player skips up to 30 days (`Wait(720)`):
   - Commodity balances update in bulk blocks.
   - Pending consequences evaluate their milestones deterministically.
   - For every matured consequence, an entry is written to `ReturnDigestLog`.
3. **Return Salutations:** Upon return to Thornveil, the first interaction with relevant NPCs triggers dialogue nodes from `ReturnDigestLog` explaining what transpired.

---

## 11. NPC Decision Integration & Goals

NPCs evaluate requests and select routines deterministically without heavy neural or utility solvers:

```rust
pub fn evaluate_assistance_request(
    npc: &CitizenMeta,
    bond: &RelationalBond,
    request_type: AssistanceType,
) -> bool {
    match bond.mode() {
        BehavioralMode::DevotedAlly => true,
        BehavioralMode::HardenedEnemy => false,
        BehavioralMode::GrudgingDebtor => bond.obligation >= request_type.cost(),
        BehavioralMode::AffectionateRefusal => {
            // Grants comfort/conversation, but refuses tools, coin, or secrets
            matches!(request_type, AssistanceType::ShareMeal | AssistanceType::CasualChat)
        },
        BehavioralMode::WaryConsultant => {
            // Grants professional/academic collaboration only
            matches!(request_type, AssistanceType::DiscussArchive | AssistanceType::InspectSpecimen)
        },
    }
}
```

---

## 12. Scholar Progression Architecture (Stage 2)

```text
Scholar Stage 1: The Inscriber
  - Capability: Inscription (Level 1)
  - Actions: Inscribe(Observation), StudyArchive
  - Role: The one who records observations

Scholar Stage 2: The Settlement Chronicler
  - Capability: Inscription (Level 2), Diagnosis
  - Actions: Diagnose(LocationId), DraftDocument(DocumentType)
  - Role: Documentary Authority (can arbitrate disputes legally)
```

### Physical Inscribed Documents
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InscribedDocument {
    pub id: u32,
    pub doc_type: DocumentType,
    pub drafter: CitizenId,
    pub signers: Vec<CitizenId>,
    pub binding_tick: u64,
    pub related_consequence_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentType {
    DebtReliefCharter { creditor: CitizenId, debtor: CitizenId, terms: u32 },
    HarvestDiagnosisReport { location: LocationId, finding: u16 },
    FoundingArchiveTranslation { secret_id: u16 },
}
```
Presenting an `InscribedDocument` signed by required parties instantly marks a `PendingConsequence` as `Resolved`, defusing social feuds.

---

## 13. Persistence Evolution: `GODSEED1` $\rightarrow$ `GODSEED2`

```rust
pub const MAGIC_V1: &[u8; 8] = b"GODSEED1";
pub const MAGIC_V2: &[u8; 8] = b"GODSEED2";

pub fn migrate_v1_to_v2(v1_snapshot: V1Snapshot) -> V2Snapshot {
    // 1. Map scalar disposition into RelationalBond(sentiment = val, trust = 0, obligation = 0)
    // 2. Initialize default EpistemicState from old KnowledgeInventory
    // 3. Initialize empty EpisodicMemory with default baseline anchor
    // 4. Initialize empty PendingConsequenceRegistry
    // 5. Emit migration audit event
}
```
Legacy saves are safely upgraded without loss; CRC32 verifies payload integrity.

---

## 14. Determinism & Invariant Suite

### Invariants (Machine-Checkable):
- **INV-1 (Population Integrity):** Total alive citizens $\in [1, 16]$; IDs unique.
- **INV-2 (Currency Conservation):** Personal coins + settlement treasury $\ge 0$.
- **INV-3 (Metabolic Bounds):** Satiety, rest, health $\in [0, 100]$.
- **INV-4 (Relational Symmetry):** No entity holds a bond with itself (`bonds.get(self.id).is_none()`).
- **INV-5 (Memory Bounds):** For all NPCs, `transient.len() <= 12` and `anchors.len() <= 6`.
- **INV-6 (Consequence Integrity):** Every `PendingConsequence` references valid alive citizens and a registered `causal_root`.
- **INV-7 (Epistemic Validity):** All knowledge IDs resolve to definitions in `ContentDefinitions`.

---

## 15. The Three Experience Proofs

1. **Proof A — One Person Matters:** Player builds deep history with Tomas Birch; past felling deed alters future assistance and dialogue.
2. **Proof B — One Consequence Lands:** Assisting Tomas displaces Runn Birch; Runn seeks forge apprenticeship; consequences unfold across weeks.
3. **Proof C — One Absence Matters:** Player waits 30 days; returns to find Runn at the forge, timber prices doubled, and Mira delivering an epistemic return salutation.

---

## 16. Thin Causal Slice (Milestone I1)

The immediate first implementation target:
- Action: Player executes `Help Felling` at West Woods with Tomas Birch.
- State: Tomas gains permanent memory anchor; bond becomes `DevotedAlly`; `CausalId` #101 emitted.
- Autonomous Continuation: `PendingConsequence::FraternalLaborStrain` registered with `causal_root = 101`.
- Progression: Simulation advances 14 days; consequence matures; Runn's schedule shifts to Forge.
- Discovery: Player talks to Tomas; Tomas's dialogue cites Root #101 via `ReturnDigestLog`.

---

## 17. Dependency-Aware Implementation Sequence

- **I1 — Causal Foundation & Thin Slice:** Causal pointers, triad bonds, episodic memory, Thin Causal Slice test harness.
- **I2 — Asymmetric Knowledge & Corroborating Gossip:** `EpistemicState`, knowledge definitions, corroborating gossip system.
- **I3 — Autonomous Situations & Absence:** `PendingConsequenceRegistry`, daily progression, macro-absence resolver, return digest.
- **I4 — Scholar Stage 2 Depth:** `Diagnose` action, `InscribedDocument`, dispute arbitration.
- **I5 — Integrated Candidate & Playtest Instrument:** Flight recorder telemetry, soak tests, human playtest candidate preparation.

---

## 18. Architectural Decision Records (ADRs)

- **ADR-001:** Compact Triad Relationship Bond $(S, T, O)$ + Derived `BehavioralMode`.
- **ADR-002:** Dual-Stream Bounded Episodic Memory (12 Transient + 6 Anchors).
- **ADR-003:** Tripartite Knowledge Representation & Epistemic State.
- **ADR-004:** Authoritative `PendingConsequenceRegistry` with Composable Triggers.
- **ADR-005:** Save Version Bump `GODSEED1` $\rightarrow$ `GODSEED2` with Upward Migration.
- **ADR-006:** SimLab Empirical Optimizations with Calibration Demotions & Cognitive Firewall.
