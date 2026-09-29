# GODSEED VERTICAL SLICE 2: PRODUCT CONTRACT
## Meaning, Attachment & Consequence in Thornveil

**Contract Identifier:** `GODSEED-VS2-CONTRACT-001`  
**Governing Phase:** Vertical Slice 2 (Product Review Baseline)  
**Status:** `READY_FOR_REVIEW`  
**Date:** 2026-09-29  
**Baseline Candidate:** `391d2937379d775d12da2425001ea2fc88b733b4` (VS1 Accepted Candidate)  

---

## 1. Mission

The mission of Godseed Vertical Slice 2 is to prove that the verified simulation machinery of Thornveil can become a living world the player actually cares about inhabiting. VS2 evaluates whether relationships, discoveries, conflicts, consequences, absences, and personal transformation can create an emergent, emotionally resonant life worth living and continuing.

---

## 2. Player Fantasy

> **The player arrives as a vulnerable, illiterate outsider in a hardscrabble settlement and becomes consequential not through physical violence or magical ascension, but through what they observe, what they record, who they stand with, whose secrets they protect, and how their documentary authority alters the social balance of an interconnected human community.**

---

## 3. Target Experience

1. **Grounded Embodiment:** Immediate vulnerability; physical survival requires food, shelter, and modest day wages.
2. **Interconnected Social Web:** Inhabitants possess private burdens, family loyalties, economic debts, and personal grievances that operate without player prompting.
3. **Qualitative Human Relationships:** Inhabitants react based on historical trust, obligation, affection, and respect rather than a single numerical score.
4. **Epistemic Agency:** Knowledge is an asymmetric world resource. Discovering hidden records, private debts, or traditional remedies gives the player unique leverage.
5. **Scholar Life Transformation:** Advancing along the Inscription path establishes documentary literacy, granting the power to draft binding contracts, survey boundaries, decipher historical truths, and arbitrate communal disputes.
6. **Living Absence & Coherent Return:** Leaving Thornveil allows ongoing disputes to progress autonomously. Returning reveals discoverable, narrative-rich consequences that demonstrate the world does not wait for its protagonist.

---

## 4. Experience Loop

```text
Observe Inhabitants & Routines
  ↓
Discover Need, Grievance, or Secret
  ↓
Choose Intervention or Allegiance
  ↓
Commit Action (Labor, Trade, Inscription, Confrontation)
  ↓
Encounter Immediate Social / Economic Reaction
  ↓
Advance Personal Scholar Trajectory
  ↓
Depart / Advance Simulated Time
  ↓
Discover Delayed Consequences & Off-Screen Evolutions
  ↓
Re-evaluate Loyalties & Inhabitant Destinies
```

---

## 5. Required Mechanics (The 5 Core Systems)

VS2 commits to building exactly five tightly bounded systems upon the VS1 simulation foundation:

1. **Multi-Dimensional Relationship Ledger:** Replaces scalar `rapport` with a 4D vector: `(Affection, Trust, Obligation, Deference)`. History alters future behavioral possibilities.
2. **Episodic Consequential Memory & Narrative Gossip:** Structured memory recording subjects, actions, salience (1–10), and emotional valence. Bounded decay for minor events; permanent retention for major turning points. Multi-hop propagation of narrative episodes through co-located socializing.
3. **Asymmetric Knowledge & Epistemic Resource System:** Distinct knowledge entities possessing custody, social danger, credibility, and prerequisites. Information asymmetry shifts social bargaining power.
4. **Inscription & Document Authority System (Scholar Stage 2):** Physical and inventory representation of drafted documents (Charters, Promissory Notes, Boundary Surveys). Empowers the player to legally bind NPCs, arbitrate property disputes, and expose or preserve historical records.
5. **Social Vector Progression Engine:** Off-screen advancement of unresolved social disputes during player absence or fast skips, generating narrative state transitions and conversational return digests.

---

## 6. Scope Boundaries

- **Setting:** Thornveil Settlement only (11 spatial nodes: Inn, Forge, Market, North Fields, South Fields, Herb Garden, Well, Old Archive, Road, Storage House, Forest Edge).
- **Population:** Exactly the 15 authored citizens from VS1. Zero procedural NPCs.
- **Duration:** 1 to 90 simulated days of inhabitation and absence.
- **Interface:** Headless simulation engine with rich terminal text RPG client.

---

## 7. Explicit Exclusions

The following systems are **STRICTLY EXCLUDED** from VS2:
- No continental expansion or secondary settlements.
- No population expansion or procedural character generation.
- No combat, weapons, health damage from violence, or military systems.
- No alternative transformation paths (no Warrior, Merchant, Mystic).
- No 2D/3D graphical clients or curses frontends.
- No non-deterministic generative LLM calls in the simulation loop.
- No procedural terrain or map generation.

---

## 8. Acceptance Criteria

| ID | Criterion Statement | Verification Mechanism |
| :--- | :--- | :--- |
| **AC-201** | **Qualitative Relational Divergence:** Two NPCs must respond differently to an identical player request based on distinct combinations of Affection, Trust, and Obligation. | Automated behavioral matrix test |
| **AC-202** | **Episodic Narrative Recall:** An event of Salience $\ge 7$ must be cited in dialogue by a witness $\ge 14$ days later, altering at least one available action. | Automated memory aging test |
| **AC-203** | **Multi-Hop Narrative Gossip:** A witnessed player action must propagate across at least two NPC socializing hops, altering a non-witness's trust prior to direct contact. | Multi-hop gossip integration test |
| **AC-204** | **Asymmetric Knowledge Leverage:** The player can acquire a documented fact unknown to a target NPC, and revealing it must alter an ongoing NPC decision. | Knowledge leverage integration test |
| **AC-205** | **Documentary Authority:** Reaching Scholar Stage 2 unlocks crafting Inscribed Documents that legally resolve an ongoing dispute between two NPCs. | Inscription contract test |
| **AC-206** | **Epistemic Backlash:** Inscribing or revealing a controversial truth causes at least one NPC's relationship to become actively hostile while another increases trust. | Social divergence test |
| **AC-207** | **Intelligible Delayed Consequence:** An action in Week 1 produces a secondary consequence in Week 3 whose causal chain is fully verifiable in telemetry. | Telemetry causal audit test |
| **AC-208** | **Autonomous Absence Continuation:** A social tension advances through $\ge 1$ major state transition during a 30-day absence, producing discoverable return consequences. | 30-day absence divergence test |
| **AC-209** | **Human Story Retelling Pass:** $\ge 3$ of 5 supervised human playtesters summarize their playthrough using characters, motives, and personal dilemmas rather than game mechanics. | Qualitative playtest audit |
| **AC-210** | **Human Curiosity Pass:** $\ge 4$ of 5 human playtesters unpromptedly inquire about the motivations, secrets, or future of at least two specific Thornveil inhabitants. | Curiosity question audit |

---

## 9. Human Playtest Requirements

1. **Cohort Size:** 3 to 5 real human players (unaffiliated with engine development).
2. **Session Length:** 2 to 4 hours of uninterrupted inhabitation.
3. **Protocol:** Blind playthrough with minimal initial instruction; no coaching or leading prompts.
4. **Instruments:**
   - Post-session Story Retelling Transcript (unprompted summary).
   - In-session Curiosity Question Log (recorded by observer).
   - Character Recall Questionnaire (identifying NPCs and their struggles).
   - Return Reaction Audit (monitoring spontaneous reactions after a 30-day absence).

---

## 10. Evidence Requirements

To qualify as a PR-ready candidate, the VS2 campaign must produce:
1. `GODSEED_VS2_ARCHITECTURE.md`: Governed architecture document addressing all architectural pressures.
2. `GODSEED_VS2_INTEGRATION_TESTS`: Automated test suite covering AC-201 through AC-208.
3. `GODSEED_VS2_HUMAN_PLAYTEST_REPORT.md`: Comprehensive qualitative evidence, transcripts, and scores from 3–5 human sessions validating AC-209 and AC-210.
4. `GODSEED_VS2_SOAK_REPORT.json`: 90-day soak test verifying memory bounds, invariant integrity, and absence continuity.

---

## 11. Stop Conditions

Development must immediately HALT and return to Product Review if:
- Human playtesters consistently describe the game in mechanical terms ("grinding inscription skill") despite relationship and memory systems.
- Multi-dimensional relationships produce combinatorial confusion rather than distinct social archetypes.
- Episodic memory growth causes unbounded RAM leakage during 90-day soak testing.
- Players express feeling cheated by delayed consequences because causality was too opaque.

---

## 12. Success Conditions

Vertical Slice 2 is declared **SUCCESSFUL** when:
1. All 10 Acceptance Criteria (AC-201 through AC-210) achieve `PASS`.
2. All 6 Fun Hypotheses (FH-201 through FH-206) are supported by empirical evidence.
3. Human playtesters demonstrate genuine attachment, retell compelling emergent stories, and express a voluntary desire to continue living in Thornveil.
