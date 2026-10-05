# GODSEED VERTICAL SLICE 2: REQUIREMENT-TO-ARCHITECTURE TRACEABILITY MATRIX

**Document Identifier:** `GODSEED-VS2-TRACE-001`  
**Governing Contract:** [GODSEED_VS2_PRODUCT_CONTRACT_REVISED.md](file:///C:/Users/15103/.gemini/antigravity/scratch/godseed/GODSEED_VS2_PRODUCT_CONTRACT_REVISED.md)  
**Governing Architecture:** [GODSEED_VS2_ARCHITECTURE.md](file:///C:/Users/15103/.gemini/antigravity/scratch/godseed/GODSEED_VS2_ARCHITECTURE.md)  
**Date:** 2026-09-29  

---

## 1. Traceability Overview

This matrix establishes complete, bidirectional traceability between the 8 approved product acceptance criteria (AC-201 through AC-208) in the Revised Product Contract and the technical systems, components, and execution paths defined in the Systems Architecture.

---

## 2. Requirement Traceability Records (AC-201 through AC-208)

### AC-201: Qualitative Relational Divergence
- **Product Requirement:** Two NPCs must exhibit distinctly different behavioral responses to the identical player request (e.g., lodging, loan, teaching) based on differing historical combinations of trust, obligation, and warmth, rather than a single rapport score.
- **Architectural Capability:** Triad Relational Bond (`RelationalBond`) & Derived Behavioral Mode Deduction (`BehavioralMode`).
- **Authoritative State:** `RelationalLedger` component on NPC entities storing `sentiment: i8`, `trust: i8`, `obligation: i16`.
- **Execution Path:**
  1. Player issues `PlayerAction::Talk` or `PlayerAction::Offer`.
  2. `PlayerActionSystem` queries target NPC's `RelationalLedger`.
  3. `RelationalBond::mode()` evaluates whether bond is `GrudgingDebtor`, `AffectionateRefusal`, `WaryConsultant`, `DevotedAlly`, or `HardenedEnemy`.
  4. `evaluate_assistance_request()` branches on mode.
- **Observable Result:** Wren (GrudgingDebtor) begrudgingly accepts a tool repair due to debt while expressing hostility; Mira (AffectionateRefusal) warmly declines a loan due to lack of trust.
- **Automated Verification:** `test_ac201_relational_divergence` (Unit & Integration).
- **Human Verification:** Observed in playtest session during player assistance negotiations.

---

### AC-202: Episodic Narrative Recall
- **Product Requirement:** A major life-altering player action must be explicitly cited by an NPC witness in dialogue at least 14 simulated days after occurrence, altering at least one available dialogue option or decision.
- **Architectural Capability:** Bounded Episodic Memory with Permanent Anchor Retention and Template Token Translation.
- **Authoritative State:** `EpisodicMemory.anchors` vector on NPC entities storing `EpisodicRecord` with `is_permanent = true` and `narrative_token`.
- **Execution Path:**
  1. Significant player action commits `EpisodicRecord` with `is_permanent = true`.
  2. 14 days advance (336 ticks); `EpisodicMemoryConsolidationSystem` prunes transient memories but preserves anchors.
  3. Player initiates `PlayerAction::Talk`.
  4. Dialogue assembly system checks `EpisodicMemory.anchors` involving `CitizenId::PLAYER` and prepends authored recall line to dialogue options.
- **Observable Result:** After 14 days, NPC explicitly states: *"I haven't forgotten that you gave the last sack of grain to Gwen while our kettle was empty."*
- **Automated Verification:** `test_ac202_episodic_recall` (advances clock 336 ticks; verifies memory presence and dialogue output string).
- **Human Verification:** Evaluated via Story Retelling Test (player references NPC recalling past events).

---

### AC-203: One-Hop Narrative Gossip
- **Product Requirement:** A high-impact player action witnessed by Citizen A must propagate to co-located Citizen B during socializing, causing Citizen B to alter their attitude or dialogue toward the player before direct contact.
- **Architectural Capability:** Weekly Narrative Gossip System.
- **Authoritative State:** `EpisodicMemory.transient` on non-witness NPC entities; `EventRing` log.
- **Execution Path:**
  1. Action witnessed by Citizen A creates permanent anchor.
  2. Weekly schedule fires `NarrativeGossipSystem`.
  3. Citizen A and Citizen B co-locate at The Slanted Timber during `NpcActivity::Socializing`.
  4. System transfers attenuated `EpisodicRecord` with tag `HeardGossipAbout(PLAYER)` to Citizen B's `EpisodicMemory`.
  5. Citizen B's `RelationalLedger.bonds[PLAYER]` shifts sentiment and trust.
- **Observable Result:** Player approaches Citizen B for the first time; Citizen B greets the player with suspicion or gratitude based on what they heard from Citizen A.
- **Automated Verification:** `test_ac203_one_hop_gossip` (places A and B at Inn, triggers gossip tick, verifies B's memory and bond).
- **Human Verification:** N/A (Automated test sufficient).

---

### AC-204: Asymmetric Knowledge Leverage
- **Product Requirement:** The player must be able to acquire a documented or observed fact unknown to a target NPC, and revealing that fact must cause the NPC to alter an ongoing economic or social decision.
- **Architectural Capability:** Asymmetric Epistemic State & Knowledge Gating System.
- **Authoritative State:** `EpistemicState.known` map on player and NPC entities.
- **Execution Path:**
  1. Player learns `KnowledgeId` (e.g., Delia's private debt or crop blight sign) via observation or archive study.
  2. Player initiates `PlayerAction::Talk` with topic `ShareKnowledge(KnowledgeId)`.
  3. `PlayerActionSystem` verifies knowledge asymmetry (player knows, NPC does not).
  4. Knowledge is inserted into NPC's `EpistemicState`.
  5. If knowledge carries a `social_fallout_mode`, target NPC's current goal or commercial term mutates immediately.
- **Observable Result:** Revealing Delia's secret ledger causes Delia to drop a surcharge or forgive an outstanding rent claim.
- **Automated Verification:** `test_ac204_asymmetric_knowledge` (verifies action availability and decision flip upon knowledge transfer).
- **Human Verification:** Playtester unpromptedly uses secrets as social leverage.

---

### AC-205: Epistemic Consequence & Scholar Agency
- **Product Requirement:** Reaching Scholar Stage 2 must unlock observational diagnosis and the crafting of Inscribed Documents that materially transform an ongoing dispute between two NPCs.
- **Architectural Capability:** Scholar Stage 2 Epistemic Actions (`Diagnose`, `InscribeDocument`) & Document Registry.
- **Authoritative State:** `TransformationState.stage = 2`, `DocumentRegistry`, `Inventory` items.
- **Execution Path:**
  1. Player meets Scholar Stage 2 prerequisites (studied archive founding stone, 5 basic inscriptions, consulted Voss).
  2. Player performs `PlayerAction::Diagnose` at North Fields, identifying blight.
  3. Player performs `PlayerAction::InscribeDocument(DebtReliefCharter)`.
  4. Document item created in player inventory and registered in `DocumentRegistry`.
  5. Player presents document to Delia and Pella; both sign (`signers.push()`).
  6. Related vector in `SocialVectorRegistry` transitions from `Active` to `Resolved`.
- **Observable Result:** Legal debt charter signed; Pella is permanently released from lodging debt; Delia ceases harassment.
- **Automated Verification:** `test_ac205_scholar_stage2_inscription` (validates diagnosis action, document creation, and dispute resolution).
- **Human Verification:** Evaluated in Story Retelling Test (player recounts using documentation to resolve a town crisis).

---

### AC-206: Intelligible Delayed Consequence
- **Product Requirement:** A player intervention in Week 1 must trigger a secondary consequence in Week 3 that was not immediately resolved upon action completion, with a fully auditable causal chain in telemetry.
- **Architectural Capability:** Social Vector Progression Pipeline with Causal Pointers.
- **Authoritative State:** `SocialVectorRegistry.vectors` with `VectorStage::Active`, `trigger_tick`, and `causal_root`.
- **Execution Path:**
  1. Week 1: Player intervention creates `SocialVector` with `trigger_tick = current_tick + 336` (14 days).
  2. Daily ticks advance; vector remains `Active`, emitting forewarning dialogue cues.
  3. Week 3: `SocialVectorProgressionSystem` detects `current_tick >= trigger_tick`.
  4. Vector state advances to `Matured`; secondary effect executed (e.g. tool price spike or labor reassignment).
  5. `TelemetryEvent::VectorMatured` emitted linking `causal_root`.
- **Observable Result:** Two weeks after aiding Oswin, player discovers tool prices doubled at Wren's forge, with Wren explicitly citing Delia's lack of cash to buy iron.
- **Automated Verification:** `test_ac206_delayed_consequence` (validates 14-day delay and causal lineage in telemetry log).
- **Human Verification:** Playtester identifies the link between their early choice and the later dilemma without confusion.

---

### AC-207: Autonomous Absence Continuation
- **Product Requirement:** An active situation involving two NPCs must advance through at least one major state transition during a 30-day player absence, producing observable physical and conversational changes upon return.
- **Architectural Capability:** Macro-Absence Vector Resolver & Return Digest System.
- **Authoritative State:** `SocialVectorRegistry`, `ReturnDigestLog`, `NpcSchedule`.
- **Execution Path:**
  1. Active vector exists (e.g., Runn's labor strain at the timber stand).
  2. Player departs / executes `Wait(720)` (30 days).
  3. `MacroAbsenceVectorResolver` advances simulation in macro blocks:
     - Vector transitions from `Active` to `Matured`.
     - Runn's `NpcSchedule` changes from Forest Edge to Wren's Forge.
     - `ReturnDigestLog` records entry for Runn's apprenticeship.
  4. Player returns to Thornveil.
  5. Inspecting forge shows Runn present; speaking with Mira triggers return digest salutation.
- **Observable Result:** The player returns to find Runn working at the blacksmith forge; NPCs comment on the change that occurred while the player was away.
- **Automated Verification:** `test_ac207_absence_continuation` (verifies 720-tick leap, schedule reassignment, and digest emission).
- **Human Verification:** Evaluated via Return Reaction Audit (player expresses surprise and curiosity at changes).

---

### AC-208: Human Story Retelling & Curiosity Gate (The Kill Test)
- **Product Requirement:** In $\ge 3$ of 5 supervised human playtests, players must unpromptedly summarize their playthrough as a human drama involving named characters and motives (rather than system mechanics), and express spontaneous curiosity about what happened during absence.
- **Architectural Capability:** Playtest Flight Recorder & Causal Narrative Audit Generator.
- **Authoritative State:** Telemetry session recording (`saves/playtest_<timestamp>.session`).
- **Execution Path:**
  1. Human player completes 2–4 hour blind play session.
  2. Flight recorder logs command sequence, seed, and causal consequence tree.
  3. Evaluator conducts unprompted interview (*"Tell me what happened in Thornveil"*).
  4. Evaluator audits transcript against Kill Test criteria; matches narrative claims against flight recorder causality log.
- **Observable Result:** $\ge 3$ of 5 players recount personal drama (e.g., *"I helped Pella with her debt, but Delia boycotted me, and when I came back Runn was working the forge"*).
- **Automated Verification:** Harness validates that flight recorder captures all required event streams and generates clean audit logs.
- **Human Verification:** **MANDATORY HUMAN PLAYTEST GATE** (supervised by human evaluation lead).

---

## 3. Reverse Traceability: Architecture to Product Requirements

Every major architectural subsystem maps directly to an approved product requirement:

| Architectural Component | Product Requirement Justification |
| :--- | :--- |
| `RelationalLedger` & `RelationalBond` | AC-201 (Qualitative Relational Divergence) |
| `EpisodicMemory` (Anchors & Transient) | AC-202 (Episodic Recall), AC-203 (Gossip) |
| `EpistemicState` & Tripartite Knowledge | AC-204 (Knowledge Leverage), AC-205 (Scholar Agency) |
| `PendingConsequenceRegistry` | AC-206 (Delayed Consequence), AC-207 (Absence Continuation) |
| `ReturnDigestLog` | AC-207 (Absence Return Experience), AC-208 (Kill Test) |
| `DocumentRegistry` | AC-205 (Documentary Authority & Scholar Stage 2) |
| `PlaytestFlightRecorder` | AC-208 (Human Playtest Gate & Causal Audit) |
| `migrate_v1_to_v2` | Persistence integrity, backward compatibility |

**Zero unmapped architectural subsystems exist.** The architecture is strictly product-bound.

---

## 4. Social Authority Remediation Trace

Remediation candidate after predecessor `3312a814ce3cdcbe4907b190c64854fc706e9f01` makes the VS2 social model canonical for runtime gameplay.

| Legacy Authority | V2 Disposition |
| :--- | :--- |
| `resources::RelationshipLedger` | Removed from V2 runtime world insertion and gameplay system signatures. Retained only in V1/V2 snapshot compatibility structs so old saves can migrate relationship values into `components::RelationalLedger`. |
| `components::NpcMemory` | No longer spawned for fresh V2 NPCs and no longer processed by scheduled gameplay systems. Retained for V1 deserialization compatibility. |
| Dynamic `components::Disposition::toward_player` | No longer used for gameplay relationship decisions. Static teaching metadata remains pending a later `TeachingProfile` split. |

Canonical runtime social decisions now derive from:

- `components::RelationalLedger`
- `components::EpisodicMemory`
- `components::EpistemicState`
- `resources::DocumentRegistry`
- `resources::PendingConsequenceRegistry`
- `resources::ReturnDigestLog`

AC-201 through AC-207 remain deterministic technical criteria. AC-208 remains deferred to supervised human evaluation.
