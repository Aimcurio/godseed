# GODSEED VERTICAL SLICE 2: REQUIREMENT-TO-ARCHITECTURE TRACEABILITY MATRIX

**Document Identifier:** `GODSEED-VS2-TRACE-001`  
**Governing Contract:** [GODSEED_VS2_PRODUCT_CONTRACT_REVISED.md](file:///C:/Users/15103/.gemini/antigravity/scratch/godseed/GODSEED_VS2_PRODUCT_CONTRACT_REVISED.md)  
**Governing Architecture:** [GODSEED_VS2_ARCHITECTURE.md](file:///C:/Users/15103/.gemini/antigravity/scratch/godseed/GODSEED_VS2_ARCHITECTURE.md)  
**Date:** 2026-10-05  

---

## 1. Traceability Overview

This matrix establishes complete, bidirectional traceability between the 8 approved product acceptance criteria (AC-201 through AC-208) in the Revised Product Contract and the technical systems, components, and execution paths defined in the Systems Architecture.

All referenced symbols, components, resources, and tests in this document exist directly in the verified codebase.

---

## 2. Requirement Traceability Records (AC-201 through AC-208)

### AC-201: Qualitative Relational Divergence
- **Product Requirement:** Two NPCs must exhibit distinctly different behavioral responses to the identical player request (e.g., lodging, loan, teaching, work) based on differing historical combinations of trust, obligation, and warmth, rather than a single rapport score.
- **Architectural Capability:** Triad Relational Bond (`RelationalBond`) & Derived Behavioral Mode Deduction (`BehavioralMode`).
- **Authoritative State:** `RelationalLedger` component on NPC entities storing `sentiment: i8`, `trust: i8`, `obligation: i16`.
- **Execution Path:**
  1. Player issues `PlayerAction::Talk { npc, topic: TalkTopic::RequestWork }` or other interaction.
  2. `player_action_system` queries target NPC's `RelationalLedger`.
  3. `RelationalBond::mode()` evaluates whether bond is `GrudgingDebtor`, `AffectionateRefusal`, `WaryConsultant`, `DevotedAlly`, or `HardenedEnemy`.
  4. Response branches directly on mode:
     - `AffectionateRefusal` (high sentiment +50, low trust -30, obligation 0): warmly refuses sensitive business/work.
     - `GrudgingDebtor` (negative sentiment -30, moderate trust 0, high obligation +60): complies begrudgingly due to obligation, deducting 20 obligation.
- **Observable Result:** A scalar rapport average would score NPC A as +10 (accept) and NPC B as -15 (reject). The multi-dimensional Godseed model strictly inverts this outcome, producing warm refusal from NPC A and begrudging compliance from NPC B.
- **Automated Verification:** `test_ac201_relational_divergence` in `crates/godseed_core/tests/social_authority_corrective.rs`.
- **Human Verification:** Observed in playtest session during player assistance negotiations.

---

### AC-202: Episodic Narrative Recall
- **Product Requirement:** A major life-altering player action must be explicitly cited by an NPC witness in dialogue at least 14 simulated days after occurrence, altering at least one available dialogue option or decision.
- **Architectural Capability:** Dual-Stream Bounded Episodic Memory with Permanent Anchor Retention (`EpisodicMemory`, Option A Compact Overflow Storage) and Template Token Translation.
- **Authoritative State:** `EpisodicMemory.anchors` vector and `EpisodicMemory.compacted_anchors` vector storing `EpisodicRecord` / `CompactAnchor` with `is_permanent = true` and `narrative_token: u16`.
- **Execution Path:**
  1. Significant player action (`PlayerAction::HelpWithFelling`) commits `EpisodicRecord` with `tag = MemoryTag::HelpedWithFelling`, `is_permanent = true`.
  2. 14 days advance ($\ge 336$ ticks).
  3. Under anchor pressure (>6 permanent anchors, >12 transients), Option A evicts lowest-magnitude active anchors into `compacted_anchors` indefinitely.
  4. Player interacts via `PlayerAction::Talk { npc: CitizenId(6), topic: TalkTopic::RequestWork }`.
  5. `has_anchor_with_tag(MemoryTag::HelpedWithFelling)` detects turning point and grants work with explicit citation: *"After what you did with the great oak, my work is always open to you."*
- **Observable Result:** After $\ge 14$ days and under severe memory pressure, NPC explicitly cites the felling assistance and accepts work even when neutral/wary.
- **Automated Verification:** `test_ac202_episodic_recall_changes_behavior` and `test_permanent_turning_point_survives_anchor_pressure` in `crates/godseed_core/tests/social_authority_corrective.rs`.
- **Human Verification:** Evaluated via Story Retelling Test (player references NPC recalling past events).

---

### AC-203: One-Hop Narrative Gossip
- **Product Requirement:** A high-impact player action witnessed by Citizen A must propagate to co-located Citizen B during socializing, causing Citizen B to alter their attitude or dialogue toward the player before direct contact.
- **Architectural Capability:** Weekly Narrative Gossip System (`gossip_system`) with Firsthand One-Hop Propagation Boundary.
- **Authoritative State:** `EpisodicMemory` on listener NPC; `RelationalLedger` on listener NPC.
- **Execution Path:**
  1. Witness A possesses firsthand episodic record regarding player (`actor == CitizenId::PLAYER || target == Some(CitizenId::PLAYER)`).
  2. Weekly schedule fires `gossip_system`.
  3. Witness A and Citizen B co-locate during `NpcActivity::Socializing`.
  4. Witness A shares firsthand experience.
  5. Citizen B receives `EpisodicRecord` with `tag: MemoryTag::HeardGossipAbout(speaker_id)` and attenuated relational bond shift.
  6. Secondhand gossip cannot be re-transmitted (one-hop bound).
  7. On first greeting, Citizen B states: *"Tomas told me what you did at the woodlot with that great oak. We can always use good hands around here."*
- **Observable Result:** Citizen B greets the player citing Tomas's story and alters interaction before any direct player contact.
- **Automated Verification:** `test_ac203_one_hop_narrative_gossip` in `crates/godseed_core/tests/social_authority_corrective.rs`.
- **Human Verification:** N/A (Automated test sufficient).

---

### AC-204: Asymmetric Knowledge Leverage
- **Product Requirement:** The player must be able to acquire a documented or observed fact unknown to a target NPC, and revealing that fact must cause the NPC to alter an ongoing economic or social decision.
- **Architectural Capability:** Asymmetric Epistemic State & Knowledge Gating System (`EpistemicState`, fail-closed sharing).
- **Authoritative State:** `EpistemicState.known` on player and NPC entities; `RelationalLedger`.
- **Execution Path:**
  1. Attempting to share unknown knowledge (`PlayerAction::Talk { topic: TalkTopic::ShareKnowledge }`) fails closed with *"You don't know enough about that to share it."*
  2. Player acquires Knowledge 6 (Delia's Hidden Debt).
  3. Before revelation, Delia Croft refuses concession on `RequestWork`.
  4. Player shares Knowledge 6 with Delia Croft.
  5. Delia's obligation increases by 80 into `BehavioralMode::GrudgingDebtor`.
  6. Delia concedes on `RequestWork`: *"Fine. I'll give you ledger work and market concession—just keep your silence about my debt."*
- **Observable Result:** Player uses debt secret as social leverage, shifting Delia into `GrudgingDebtor` and granting market and ledger work concessions.
- **Automated Verification:** `test_share_unknown_knowledge_fails_closed` and `test_ac204_asymmetric_knowledge_leverage` in `crates/godseed_core/tests/social_authority_corrective.rs`.
- **Human Verification:** Playtester unpromptedly uses secrets as social leverage.

---

### AC-205: Epistemic Consequence & Scholar Agency
- **Product Requirement:** Reaching Scholar Stage 2 must unlock observational diagnosis and the crafting of Inscribed Documents that materially transform an ongoing dispute between two NPCs.
- **Architectural Capability:** Scholar Stage 2 Epistemic Actions (`Diagnose`, `InscribeDocument`) & Document Registry.
- **Authoritative State:** `TransformationState.stage = 2`, `DocumentRegistry`, `Inventory` items.
- **Execution Path:**
  1. Player meets Scholar Stage 2 prerequisites (studied archive founding stone, 5 basic inscriptions, consulted Voss).
  2. Player performs `PlayerAction::Diagnose` at North Fields, identifying blight.
  3. Player performs `PlayerAction::InscribeDocument`.
  4. Document item created in player inventory and registered in `DocumentRegistry`.
  5. Player arbitrates dispute between affected citizens; signatures recorded on document.
  6. Dispute transitions to resolved state.
- **Observable Result:** Legal debt charter / arbitration signed; dispute permanently resolved.
- **Automated Verification:** `crates/godseed_core/tests/scholar_stage2_arbitration.rs`.
- **Human Verification:** Evaluated in Story Retelling Test (player recounts using documentation to resolve a town crisis).

---

### AC-206: Intelligible Delayed Consequence
- **Product Requirement:** A player intervention in Week 1 must trigger a secondary consequence in Week 3 that was not immediately resolved upon action completion, with a fully auditable causal chain in telemetry.
- **Architectural Capability:** Causal Consequence Pipeline with Root Event Pointers (`PendingConsequenceRegistry`, `EventRing`).
- **Authoritative State:** `PendingConsequenceRegistry.consequences` with `stage: ConsequenceStage`, `causal_root: u64`.
- **Execution Path:**
  1. Root Action: `PlayerAction::HelpWithFelling { npc: CitizenId(6) }`.
  2. Root Event: `SimEvent::CausalAction { causal, action_name: "HelpWithFelling" }` emitted to `EventRing` with unique `causal.root_event_id`.
  3. Consequence tracked in `PendingConsequenceRegistry` with `cons.causal_root == causal.root_event_id` in stage `Active`.
  4. 14 days advance (336 ticks); consequence matures to `ConsequenceStage::Matured`.
  5. Secondary effect executes: Runn Birch transfers from West Woods woodlot to Apprentice Artisan at Wren's Forge (LocationId 2).
  6. Dialogues reflect the economic/social ripple (Tomas Birch, Runn Birch, Mira Ashbridge).
- **Observable Result:** Complete unbroken lineage from root action to root event ID to pending consequence to matured consequence to physical occupation and dialogue transformation.
- **Automated Verification:** `test_ac206_causal_trace_lineage` in `crates/godseed_core/tests/social_authority_corrective.rs` and `crates/godseed_core/tests/thin_causal_slice.rs`.
- **Human Verification:** Playtester identifies the link between their early choice and the later dilemma without confusion.

---

### AC-207: Autonomous Absence Continuation
- **Product Requirement:** An active situation involving two NPCs must advance through at least one major state transition during a 30-day player absence, producing observable physical and conversational changes upon return.
- **Architectural Capability:** Long-Horizon Progression & Return Digest System (`ReturnDigestLog`).
- **Authoritative State:** `PendingConsequenceRegistry`, `ReturnDigestLog`, `NpcSchedule`.
- **Execution Path:**
  1. Active consequence exists.
  2. Player departs / advances 30 days (720 ticks).
  3. Simulation advances autonomously in absence:
     - Consequence matures.
     - Schedule and occupation reassign.
     - `ReturnDigestLog` stores return digest messages.
  4. Player returns to Thornveil.
  5. Speaking with citizens drains return digests and presents narrative recaps.
- **Observable Result:** Physical and conversational world state advances during player absence; return salutations inform the player of developments.
- **Automated Verification:** `crates/godseed_core/tests/macro_absence_proof_c.rs`.
- **Human Verification:** Evaluated via Return Reaction Audit (player expresses surprise and curiosity at changes).

---

### AC-208: Human Story Retelling & Curiosity Gate (The Kill Test)
- **Product Requirement:** In $\ge 3$ of 5 supervised human playtests, players must unpromptedly summarize their playthrough as a human drama involving named characters and motives (rather than system mechanics), and express spontaneous curiosity about what happened during absence.
- **Architectural Capability:** Telemetry Flight Recorder & Causal Narrative Audit Generator.
- **Authoritative State:** `TelemetryLog`.
- **Execution Path:**
  1. Human player completes 2–4 hour blind play session.
  2. Flight recorder logs command sequence, seed, and causal consequence tree.
  3. Evaluator conducts unprompted interview (*"Tell me what happened in Thornveil"*).
  4. Evaluator audits transcript against Kill Test criteria; matches narrative claims against flight recorder causality log.
- **Observable Result:** $\ge 3$ of 5 players recount personal drama unprompted.
- **Automated Verification:** Harness validates that flight recorder captures all required event streams and generates clean audit logs.
- **Human Verification:** **MANDATORY HUMAN PLAYTEST GATE** (supervised by human evaluation lead).
- **Current Status:** `DEFERRED_HUMAN_VALIDATION_PENDING` (Pending human playtest sessions).

---

## 3. Reverse Traceability: Architecture to Product Requirements

| Architectural Component | Product Requirement Justification |
| :--- | :--- |
| `RelationalLedger` & `RelationalBond` | AC-201 (Qualitative Relational Divergence) |
| `EpisodicMemory` (Active Anchors & `compacted_anchors`) | AC-202 (Episodic Recall), AC-203 (Gossip) |
| `EpistemicState` & Tripartite Knowledge | AC-204 (Knowledge Leverage), AC-205 (Scholar Agency) |
| `PendingConsequenceRegistry` | AC-206 (Delayed Consequence), AC-207 (Absence Continuation) |
| `ReturnDigestLog` | AC-207 (Absence Return Experience), AC-208 (Kill Test) |
| `DocumentRegistry` | AC-205 (Documentary Authority & Scholar Stage 2) |
| `TelemetryLog` | AC-208 (Human Playtest Gate & Causal Audit) |
| `NpcSocialProfile` | Architectural decoupling: clean static social parameters |
| `migrate_v1_to_v3`, `migrate_v2_to_v3` | Save format V3 clean persistence, backward compatibility |

**Zero unmapped architectural subsystems exist.** The architecture is strictly product-bound.

---

## 4. Single Social Authority Closure Trace

| Entity / Resource | Status | Corrective Remediation Disposition |
| :--- | :--- | :--- |
| `resources::RelationshipLedger` | REMOVED | Never inserted into ECS world in fresh simulation or loaded snapshot. Save struct field omitted in V3 snapshots. Contradictory legacy values proven to have 0% runtime influence (`test_legacy_positive_cannot_override_vs2_enemy`, `test_legacy_negative_cannot_override_vs2_ally`). |
| `components::NpcMemory` | REMOVED | Zero entities spawned with `NpcMemory`. Omitted from V3 snapshots. Zero runtime usage. |
| `components::Disposition` | REPLACED | Replaced in gameplay and snapshots by immutable `NpcSocialProfile` (`base_personality`, `base_suspicion`, `will_teach`, `teach_threshold`). Legacy struct retained purely for V1/V2 save deserialization. |
| `components::RelationalLedger` | AUTHORITATIVE | 100% authoritative for all relational evaluations, dialogue branches, trade concessions, and teaching unlock checks. |
| `components::EpisodicMemory` | AUTHORITATIVE | Option A compact storage implemented (`compacted_anchors`); permanent turning points survive indefinite memory pressure. |
| `components::EpistemicState` | AUTHORITATIVE | Fail-closed sharing enforced; asymmetric debt leverage implemented. |
