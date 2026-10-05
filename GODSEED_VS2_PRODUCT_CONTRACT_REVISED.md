# GODSEED VERTICAL SLICE 2: REVISED PRODUCT CONTRACT
## Meaning, Attachment & Consequence in Thornveil

**Contract Identifier:** `GODSEED-VS2-CONTRACT-REVISED-002`  
**Governing Phase:** Vertical Slice 2 (Approved Product Contract Baseline)  
**Status:** `APPROVED_WITH_REVISIONS`  
**Date:** 2026-09-29  
**Baseline Candidate:** `391d2937379d775d12da2425001ea2fc88b733b4` (VS1 Accepted Candidate)  
**Predecessor Proposal:** `GODSEED_VS2_PRODUCT_CONTRACT.md` (SHA-256: `057DE261CF0BD47EF95D828C11D1F7949C3E25F644261904609A3ADE42635C55`)

---

## 1. Mission

The mission of Godseed Vertical Slice 2 is to prove that the verified simulation machinery of Thornveil can create an emergent, emotionally resonant life where the player forms real attachments and rivalries, wields asymmetric knowledge as social power, alters the destinies of autonomous inhabitants, encounters intelligible delayed consequences, and experiences an evolving world that moves without them.

---

## 2. Player Fantasy

> **The player arrives as a vulnerable, illiterate outsider in an autonomous settlement and becomes consequential not through physical violence or magical ascension, but through what they observe, what they record, who they stand with, whose secrets they protect, and how their epistemic authority alters the balance of an interconnected human community.**

---

## 3. Target Experience

1. **Grounded Embodiment:** Immediate vulnerability; physical survival requires food, shelter, and honest labor.
2. **Interconnected Social Web:** Inhabitants possess private burdens, family loyalties, economic debts, and personal grievances that operate without player prompting.
3. **Qualitative Human Relationships:** Inhabitants react based on historical trust, obligation, affection, and respect rather than a single numerical score.
4. **Epistemic Agency:** Knowledge is an asymmetric world resource. Discovering hidden records, private debts, or traditional remedies gives the player unique leverage.
5. **Scholar Life Transformation:** Advancing along the Inscription path establishes documentary literacy and observational insight, granting the power to diagnose systemic problems, decipher historical truths, draft binding agreements, and arbitrate communal disputes.
6. **Living Absence & Coherent Return:** Leaving Thornveil allows active situations to progress autonomously. Returning reveals discoverable, narrative-rich consequences that demonstrate the world does not wait for its protagonist.

---

## 4. Universal Experience Loop

```text
Observe & Connect
  ↓
Encounter Tension (Grievance, Debt, Secret, Need)
  ↓
Intervene or Refrain (Action, Labor, Inscription, Exposure)
  ↓
Live with the Immediate Ripple (Social/Economic Shift)
  ↓
Experience the Absent World (Time Skip or Departure)
  ↓
Reap the Delayed Consequence (Return, Confrontation, Transformed Destiny)
```

---

## 5. Required Core Systems (Reduced & Implementation-Agnostic)

VS2 commits to delivering four core experiential systems built upon the VS1 simulation foundation, stripping out premature architectural prescriptions:

1. **Qualitative Relational Dynamics:** Replaces single-scalar rapport with multi-faceted relational behaviors. Inhabitants must distinguish between warmth, trust, and obligation. (e.g., an enemy bound by debt renders reluctant aid; a warm friend who distrusts player discretion withholds secrets).
2. **Episodic Consequential Memory & Narrative Gossip:** Inhabitants retain permanent episodic memories of high-impact turning points (betrayals, rescues, pacts) and cite them directly in dialogue. Witnessed dramatic events propagate to co-located NPCs via one-hop narrative gossip, altering non-witness behavior before direct contact.
3. **Asymmetric Knowledge & Epistemic Resource System:** Distinct knowledge entities spanning practical insights, secret truths, and documented records. Inhabitants cannot act on knowledge they do not possess; information asymmetry alters social bargaining and decision power.
4. **Epistemic Transformation & Inscription System (Scholar Stage 2):** Unlocks deep observational diagnoses (crop blight, structural fatigue), deciphering ancient archive records, and crafting Inscribed Documents (deeds, agreements, recipes). Elevates the player to an intellectual and documentary authority in Thornveil.
5. **Autonomous Off-Screen Continuation:** Unresolved social and economic vectors progress coherently during player absence or fast skips, generating observable physical changes and narrative return salutations.

---

## 6. Scope Boundaries & Inhabitant Tiering

- **Setting:** Thornveil Settlement only (11 spatial nodes: Inn, Forge, Market, North Fields, South Fields, Herb Garden, Well, Old Archive, Road, Storage House, Forest Edge).
- **Population:** Exactly the 15 authored citizens from VS1, structured into three achievable design tiers:
  - **Anchor Tier (4 Inhabitants):** Elder Voss, Delia Croft, Wren Forscythe, Mira Ashbridge (Central political, commercial, craft, and tavern figures; full episodic depth).
  - **Focal Tier (5 Inhabitants):** Oswin Cley, Sera Cley, Pella, Tomas Birch, Runn Birch (Agricultural, herbal, labor, and fraternal conflicts; prime intervention targets).
  - **Civic Texture Tier (6 Inhabitants):** Aldous Minner, Gwen Minner, Corva, Bard Tholl, Nissa Tholl, Harwin Croft (Economic grounding, routines, market trade, and casual gossip).
- **Interface:** Headless simulation engine driving a high-clarity terminal text RPG client.

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
| **AC-201** | **Qualitative Relational Divergence:** Two NPCs must exhibit distinctly different behavioral responses to the identical player request (e.g., lodging, loan, teaching) based on differing historical combinations of trust, obligation, and warmth, rather than a single rapport score. | Automated behavioral matrix test |
| **AC-202** | **Episodic Narrative Recall:** A major life-altering player action must be explicitly cited by an NPC witness in dialogue at least 14 simulated days after occurrence, altering at least one available dialogue option or decision. | Automated memory aging test |
| **AC-203** | **One-Hop Narrative Gossip:** A high-impact player action witnessed by Citizen A must propagate to co-located Citizen B during socializing, causing Citizen B to alter their attitude or dialogue toward the player before direct contact. | Gossip propagation test |
| **AC-204** | **Asymmetric Knowledge Leverage:** The player must be able to acquire a documented or observed fact unknown to a target NPC, and revealing that fact must cause the NPC to alter an ongoing economic or social decision. | Knowledge leverage integration test |
| **AC-205** | **Epistemic Consequence & Scholar Agency:** Reaching Scholar Stage 2 must unlock observational diagnosis and the crafting of Inscribed Documents that materially transform an ongoing dispute between two NPCs. | Inscription agency test |
| **AC-206** | **Intelligible Delayed Consequence:** A player intervention in Week 1 must trigger a secondary consequence in Week 3 that was not immediately resolved upon action completion, with a fully auditable causal chain in telemetry. | Telemetry causal audit test |
| **AC-207** | **Autonomous Absence Continuation:** An active situation involving two NPCs must advance through at least one major state transition during a 30-day player absence, producing observable physical and conversational changes upon return. | 30-day absence divergence test |
| **AC-208** | **Human Story Retelling & Curiosity Gate:** In $\ge 3$ of 5 supervised human playtests, players must unpromptedly summarize their playthrough as a human drama involving named characters and motives (rather than system mechanics), and express spontaneous curiosity about what happened during absence. | Supervised playtest qualitative audit |

---

## 9. The Kill Test

```text
A blind human playtester plays Godseed for 3 simulated weeks, makes at least one
consequential intervention in a Thornveil household's affairs, leaves the
settlement for 30 simulated days, returns, and completes the session.

THE TEST PASSES IF AND ONLY IF:
1. Upon return, the player immediately seeks out the people affected by their
   prior choice to see what became of them.
2. In the unprompted post-session retelling, the player recounts their playthrough
   as a personal human drama involving named characters, motives, and regrets,
   WITHOUT referencing simulation meters or game systems.
3. The player expresses a spontaneous desire to return to the world to resolve an
   unfinished situation.

IF THE PLAYER DESCRIBES THE GAME AS MANAGING STATS, TREATS THE NPCS AS VENDORS,
OR SHOWS NO CURIOSITY ABOUT WHAT HAPPENED DURING ABSENCE, VS2 IS KILLED.
```

---

## 10. Explicit Failure Conditions

VS2 is declared a **FAILURE** if:
1. In $\ge 3$ of 5 human playtests, players describe their experience in system terminology (*"grinding rapport"*, *"leveling inscription"*, *"managing food meters"*).
2. In $\ge 3$ of 5 human playtests, players cannot name the inhabitants they interacted with.
3. Players encounter delayed consequences and perceive them as random or unfair bugs.
4. Players find the Scholar trajectory boring or bureaucratic and actively avoid it.
5. Returning after absence produces no spontaneous player exploration or curiosity.

---

## 11. Human Playtest Protocol

1. **Cohort Size:** 3 to 5 real human players (unaffiliated with engine development).
2. **Session Length:** 2 to 4 hours of uninterrupted inhabitation.
3. **Protocol:** Blind playthrough with minimal initial instruction; zero evaluator coaching.
4. **Primary Evaluation Instruments:**
   - Unprompted Post-Session Story Retelling Transcript.
   - In-Session Curiosity Question Log.
   - Character Recall Questionnaire.
   - Return Reaction Audit.

---

## 12. Evidence & Governance Deliverables

To qualify for independent acceptance, the VS2 campaign must produce:
1. `GODSEED_VS2_ARCHITECTURE.md`: Technical architecture resolving all identified architectural pressures.
2. `GODSEED_VS2_INTEGRATION_TESTS`: Automated test suite covering AC-201 through AC-207.
3. `GODSEED_VS2_HUMAN_PLAYTEST_REPORT.md`: Full transcripts, recordings, and qualitative audit logs validating AC-208 and the Kill Test across 3–5 human testers.
4. `GODSEED_VS2_SOAK_REPORT.json`: 90-day soak test verifying memory bounds, invariant integrity, and absence continuity.
