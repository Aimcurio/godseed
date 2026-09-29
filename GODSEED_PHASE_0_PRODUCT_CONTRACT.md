# GODSEED PHASE 0 — VERTICAL SLICE PRODUCT CONTRACT

**Status**: `CONTRACT_FROZEN`
**Document Control**: Phase 0 Gate — must pass before any implementation begins
**Date Frozen**: 2026-09-29
**Revision**: 1.0.0
**Authorized by**: Godseed Campaign Executor

---

## 0. WORKSPACE CONTEXT

This contract governs Godseed Vertical Slice 1 (GS-VS1). It inherits and extends the
proven simulation primitives from **CIVITAS-1M** (commit `0915514`, branch
`feature/civitas-1m-core`, `C:\Users\15103\.gemini\antigravity\scratch\civitas`) —
a production-quality Rust/Bevy-ECS simulation engine with:

- Individual persistent entity model (ECS, bincode persistence, CRC32 integrity)
- Deterministic multirate scheduler (Daily / Weekly / Monthly)
- Economy (price discovery, wages, resource flow)
- Household and demographics systems
- Save/load, replay, invariant auditing

Godseed VS1 **extends** CIVITAS-1M's simulation substrate with:

- A player entity injected into the simulation world
- NPC memory, relationship, and reputation layers
- A capability / skill acquisition system
- One transformation pathway
- A text-terminal interface enabling real player interaction
- A headless scripted-persona harness enabling automated life tests

CIVITAS-1M systems that conflict with the Godseed product thesis may be adapted
or replaced. Adapt conservatively; preserve proven invariants.

---

## 1. PLAYER FANTASY

The player believes they are:

> A living person inside a world that genuinely does not revolve around them —
> a world where other people have lives, needs, relationships, memories, and
> agendas entirely independent of the player. The player's choices ripple through
> this world in persistent, sometimes unexpected ways. The player can grow from
> nobody into something the settlement has never seen — but only if they actually
> earn it through real interactions, real work, real relationships, and real
> consequences.

The player should never feel they are completing a checklist. They should feel
they are making choices that matter because other people remember, react, and
change accordingly.

---

## 2. PLAYER ROLE

**Setting**: Thornveil — a small, self-sufficient highland settlement of ~30 inhabitants.

**Initial Player Role**:
- New arrival (outsider). No prior relationships. No special skills. Modest starting resources (5 coins, a worn tool, travel provisions for 3 days).
- Known to no one. Regarded with neutral curiosity by default.
- Must earn trust, find a place to sleep, find food, find purpose.

**Player Representation in Simulation**:
- The player is one entity in the Bevy ECS world with all the same component types
  as NPCs: PhysicalNeeds, PersonalFinances, OccupationProfile, Kinship, etc.
- **Unique player components**: `PlayerMarker`, `PlayerInputBuffer`, `CapabilitySet`,
  `TransformationState`, `KnowledgeInventory`.
- Player is not "special" in the simulation logic — the same systems that run NPCs
  also run the player's physiology, economy, and social state, so consequences
  are structurally equivalent.

---

## 3. CORE LOOP

**Primary Loop (repeatable, ~5–30 minutes per cycle)**:

```
Live → Encounter pressure or opportunity → Choose → Act → Gain or lose
capability / resources / relationships → Alter self or world state →
Experience consequences → Continue living
```

**Pressures**:
- Physical (hunger, fatigue, weather)
- Social (reputation decay, relationship obligations)
- Economic (running out of money, goods needed)
- Temporal (world events, NPC schedules creating time windows)

**Opportunities**:
- NPC needs you can fulfill
- Resources available to claim or earn
- Skills NPCs can teach
- Items or knowledge to discover
- Transformation-relevant discoveries

**Key Design Rule**: Every pressure and every opportunity must be structurally
produced by simulation systems — not authored quest scripts. Authored content
provides texture on top of systemic behavior, not a replacement for it.

---

## 4. MINUTE-TO-MINUTE PLAY

An excellent 5-minute sequence during early play:

> The player has been in Thornveil for two days. Satiety is at 62% and dropping.
> Looking around, they see Wren (blacksmith) hammering at the forge, Oswin
> (farmer) heading toward the south field, and Mira (innkeeper) wiping down
> the common room tables.
>
> The player approaches Mira and initiates a conversation. Mira offers food in
> exchange for help stacking firewood — 2 hours of labor for a hot meal plus
> 1 coin. The player accepts.
>
> During the labor, the player observes that the firewood pile is nearly
> exhausted — a systemic signal. They also notice Wren emerging from the forge
> scowling (Wren's furnace is running low on fuel). The player mentally files this.
>
> After completing the task, satiety rises to 88%. Mira's relationship score with
> the player improves fractionally. The player now has 6 coins. They decide to
> spend 30 minutes exploring the forest to look for timber — not because a quest
> marker told them to, but because the simulation revealed a resource gap.

This is the texture of play: observation → inference → choice → consequence.

---

## 5. THIRTY-MINUTE EXPERIENCE

An excellent early 30-minute session (player still largely insignificant):

The player arrived yesterday. They have:
- Survived the first night by negotiating temporary lodging with the innkeeper
  in exchange for a small task.
- Noticed that the settlement has a regular morning market.
- Learned one NPC's name and something about their routine.
- Made one choice that produced a small consequence (helped or didn't help
  someone, acquired something, said something that affected how one NPC sees them).

The player does not yet understand all the systems. They are discovering
the texture of the world through genuine exploration. They feel the world
is coherent and persistent — NPCs do not reset, consequences endure, and
the economy continues without them.

By the end of 30 minutes, the player should have at least one concrete goal
they chose themselves, formed from their own observation of the world.

---

## 6. MULTI-HOUR ARC

After several hours of play:

- The player has established a niche in Thornveil (an occupation or role).
- They have meaningful relationships with 3–8 NPCs (some positive, some neutral
  or negative).
- Their reputation is known to most inhabitants (possibly different reputations
  with different social circles).
- They have accumulated capabilities that were not available at the start.
- They have caused at least one persistent change in the settlement that other
  NPCs acknowledge or respond to.
- They are pursuing or have completed the first stage of the transformation path.

The player's choices have forked the world from what it would have been without
them — and the player can see evidence of this fork.

---

## 7. LONGITUDINAL ARC (Architecture Direction — Not Built Now)

The same systems that support the 30-minute arc extend toward:

- Deep transformation: the player has become something the settlement has never seen.
  Other NPCs react differently. New capabilities open. Old social structures shift.
- Settlement evolution: the player's economic, social, or physical interventions
  have visibly altered the settlement's composition, economy, or social structure.
- Cultural memory: NPCs refer to earlier events; younger NPCs may not remember but
  older ones do; lore propagates.
- Consequence cascade: choices from hours ago continue rippling in new ways.

The architecture must permit these without requiring a rewrite.

---

## 8. PROGRESSION MODEL

### 8.1 Personal Capability Progression

- Represented by `CapabilitySet`: a map of `CapabilityId -> CapabilityLevel`
- Capabilities are acquired by: practicing, observing, being taught, or
  through transformation.
- Examples: `Woodcutting`, `Smithing`, `Persuasion`, `Herbalism`, `Cooking`,
  `Tracking`, `Inscription` (the transformation's early stage).
- Capabilities unlock new action options (not just stat bonuses).
- A player without `Smithing` cannot perform smithing actions; a player
  with it can.

### 8.2 Social Progression

- Represented by a `RelationshipLedger`: per-NPC relationship value (-100 to +100)
  and a `SettlementReputation` aggregate per social group.
- Relationship changes are caused by: direct interaction outcomes, observed
  player behavior (NPCs gossip), resource exchanges, kept/broken commitments.
- Reputation affects: what actions NPCs are willing to offer/accept, initial
  disposition in new interactions, prices in the economy.

### 8.3 Knowledge Progression

- Represented by `KnowledgeInventory`: a set of known `KnowledgeNode` identifiers.
- Knowledge nodes include: NPC facts (name, occupation, relationships),
  world facts (resource locations, settlement layout), capability prerequisites,
  transformation clues.
- Knowledge affects: available conversation options, ability to make informed
  economic choices, ability to pursue transformation.

### 8.4 Resource/Economic Progression

- Inherited from CIVITAS-1M economy: coins, resources (food, timber, stone,
  tools, luxury), market prices.
- Player participates in the same economy as NPCs.
- Resource accumulation enables new actions (purchase, barter, gifting,
  crafting) and contributes to social standing.

### 8.5 World Progression

- The settlement evolves with or without the player's direct intervention.
- Time-skipping (rest/sleep) advances the simulation by a configurable tick count.
- NPC births, deaths, job changes, relationship changes, price shifts occur
  continuously.
- Player absence (time advancement) is observable on return.

### 8.6 Transformation Progression

- One transformation path: **The Inscription Path** (thematic: a player who
  records, studies, experiments, and eventually gains a form of persistent
  knowledge beyond normal human limits).
- Three stages (only Stage 1 must be fully implemented in VS1; Stages 2–3
  must be architecturally seamed):
  - **Stage 1 — Scholar**: Player acquires `Inscription` capability, can
    record world observations, gains knowledge bonuses, NPCs see them
    as "the one who writes things down."
  - **Stage 2 — Archivist** (seamed): Player's inscriptions form a persistent
    archive that NPCs can reference; memory extends beyond normal human limits.
  - **Stage 3 — Living Record** (seamed, conceptual): Player becomes something
    more — capable of preserving and transmitting knowledge in ways that
    structurally change how the settlement accumulates information.
- Transformation is not a stat bonus. It unlocks new action types, changes NPC
  reactions, and alters what the player can observe and affect.

---

## 9. CONSEQUENCE MODEL

Events that may persist after player actions:

| Action | Persisted Consequence |
|--------|----------------------|
| Helped NPC | Relationship improvement, possible future reciprocation |
| Harmed NPC | Relationship damage, potential reputation damage, NPC avoidance |
| Stole resource | Suspicion state on relevant NPC if witnessed |
| Taught or learned | Capability state change (both parties) |
| Completed task for NPC | Relationship improvement, possible resource reward |
| Made trade | Inventory changes both parties; price signals in market |
| Damaged property | World state change; NPC reaction if observed |
| Inscribed knowledge | Knowledge archive entry; persists in save |
| Transformation event | Permanent player state change; NPC reaction history |
| Player absence (time skip) | World continues; NPCs age, die, are born; economy shifts |
| Player reputation event | Settlement reputation affects all future dispositions |

All consequences persist through save/load cycles.

---

## 10. FAILURE AND RECOVERY

Failure is not only death. Failure states include:

| Failure Type | Mechanism | Recovery Path |
|-------------|-----------|---------------|
| **Starvation** | Satiety → 0, health collapses | Acquire food immediately or die |
| **Economic collapse** | Savings → 0, no income | Labor for coin, barter, accept charity |
| **Social failure** | Reputation critically low | Long-term relationship rebuilding |
| **Relationship breakdown** | Key NPC relationship < threshold | Apology, compensatory action, or permanent loss |
| **Physical injury** | Health < 50 from hazard | Rest, seek healer, use herbalism |
| **Opportunity loss** | Time window closed (NPC died, traded, left) | Irreversible; accept and adapt |
| **Failed experiment** | Transformation attempt fails | No transformation progress; resource spent |
| **Damaged reputation** | Witnessed transgression | Partial recovery possible; some permanent |
| **Transformation reversal** | Stage 1 incomplete; relevant NPC dies | Must find alternative teacher |

**Death handling**: Player death triggers a structured choice: reload last save,
or continue as a "new arrival" (fresh player entity) in the same persistent world
with the same consequences in place. This supports replayability while making
consequences legible.

---

## 11. FUN HYPOTHESES

These are falsifiable claims about why this slice will be enjoyable.
They must be tested during the Fun Gate phase.

**FH-1 (Discovery)**: Players will voluntarily explore NPC routines and
relationships because the world reveals itself through observation rather than
tutorials. Evidence: average time before first unsolicited exploration of NPC
routine in scripted life tests.

**FH-2 (Consequence Readability)**: Players will understand what caused their
consequences without explicit tooltips because the simulation's causal trace
system makes consequences legible. Evidence: consequence cause correctly inferred
in scripted life tests without causal display.

**FH-3 (Pressure-Driven Agency)**: Players will form their own goals from
simulation-generated pressure rather than needing authored quests. Evidence:
scripted personas generate meaningfully different goals from the same starting state.

**FH-4 (Systemic Surprise)**: Players will encounter unexpected but coherent
NPC behaviors (economic decisions, relationship shifts, death, migration) that
produce surprise without feeling arbitrary. Evidence: life test produces at least
one unscripted emergent event per run.

**FH-5 (Transformation Meaningfulness)**: The Inscription transformation will
feel meaningfully different from "level up" because it unlocks real new action
types and visibly changes NPC behavior. Evidence: post-transformation capability
comparison shows concretely new options available.

**FH-6 (Persistence Satisfaction)**: Players will feel the world is real because
returning after time advancement shows genuine persistent changes. Evidence:
return-after-skip test shows at least 3 observable world changes without player
action.

---

## 12. SCOPE EXCLUSIONS

The following are explicitly OUT of scope for VS1:

| Item | Reason |
|------|--------|
| 3D graphical rendering | Not required to test thesis; terminal UI sufficient |
| Multiple settlements | Depth before breadth |
| Multiple transformation paths | Architecture seam is enough; prove depth with one |
| Multiplayer | Premature complexity |
| Authored quests / quest scripting system | Systemic emergence is the thesis |
| Combat system | Not required for Inscription path thesis |
| Continent-scale world | One settlement; surrounding immediate area only |
| Hundreds of NPCs | 25–35 inhabitants; quality over quantity |
| Cinematic sequences | Not required |
| Music / sound effects | Text terminal; not required for thesis |
| LLM-generated NPC dialogue | Deterministic rules; reduces reproducibility |
| Modding runtime | Premature |
| Cloud / network services | Not required |

---

## 13. ACCEPTANCE CRITERIA

Each criterion is observable and testable. Evidence states:
`VERIFIED` / `SUPPORTED` / `PARTIAL` / `UNVERIFIED` / `CONTRADICTED`

### AC-1: Settlement Existence
- The world contains exactly one named settlement (Thornveil).
- Thornveil contains 25–40 persistent NPC entities.
- NPCs have named identities, occupations, households, physical needs.

### AC-2: Player Entity
- One player entity exists in the ECS world.
- Player entity shares physiology, economy, and social systems with NPCs.
- Player entity has unique: CapabilitySet, TransformationState, KnowledgeInventory, PlayerInputBuffer.

### AC-3: Movement and Physical Interaction
- Player can navigate the settlement map via text commands.
- Player can interact with objects (pick up, put down, use).
- Player can enter and exit structures (inn, forge, fields).

### AC-4: NPC Autonomy
- NPCs run routines (schedule-driven location/activity changes) each sim tick.
- NPCs have physical needs (satiety, shelter) that drive behavior.
- NPCs respond to player presence (acknowledge, offer interaction, react to reputation).

### AC-5: NPC Memory
- NPCs remember at least one player interaction per NPC.
- NPC memory affects subsequent interaction disposition.
- NPC memory persists through save/load.

### AC-6: Relationships
- Player-NPC relationship values change through interaction.
- Relationship value affects what actions NPCs offer.
- At least one relationship threshold (e.g., trust > 70) unlocks a gated action.

### AC-7: Economy / Resource Flow
- Economy continues without player participation.
- Player can earn coins through labor.
- Player can spend coins in the market.
- Player can barter goods.
- Market prices respond to supply/demand.
- Player cannot duplicate resources (economy exploit resistance).

### AC-8: Capability Acquisition
- Player can acquire at least 2 capabilities during a normal playthrough.
- Acquired capability enables at least one new concrete action not previously possible.
- Capability persists through save/load.

### AC-9: Transformation Path
- The Inscription path has at least 3 observable steps.
- Each step requires specific prerequisite conditions.
- Completing Stage 1 produces at least 2 concrete changes: one capability, one NPC reaction.
- Transformation state persists through save/load.

### AC-10: Persistence
- Save writes a valid snapshot.
- Load restores the world including: player state, NPC states, relationships, economy, time.
- At minimum 10 ticks after reload produce the same state hash as 10 ticks without intervening save/load.

### AC-11: Time Progression
- Sleeping/resting advances time by configurable tick counts.
- NPC schedules, needs, and economic state advance during time skip.
- World is demonstrably different after a 30-day time skip vs. not.

### AC-12: Persistent Consequences After Absence
- After a 30-day time skip, at least 3 observable world changes can be identified
  (NPC relationship shift, economic price change, NPC age/health change, NPC death, etc.).

### AC-13: Multiple Meaningful Playthroughs
- Running 5 scripted personas through the same world produces at least 3 measurably
  different final states (relationship divergence, economic divergence, capability divergence).

### AC-14: Fun Hypotheses Testable
- Life tests produce evidence for or against each of FH-1 through FH-6.

---

## 14. TECHNICAL CONSTRAINTS

- **Language**: Rust (2021 edition)
- **Simulation Engine**: Bevy ECS (headless, `bevy_ecs = "0.15.x"`)
- **Persistence**: bincode + CRC32 (inherited from CIVITAS-1M)
- **Determinism**: ChaCha8Rng; identical seed + tick → identical hash for all NPC behavior
- **Player Input**: Non-deterministic (explicitly separated from deterministic NPC systems)
- **Interface**: Text terminal (crossterm or similar)
- **Target Platform**: Windows x86_64 (AMD Ryzen 9, 32 GB RAM)
- **Build Tool**: cargo (workspace)
- **Test Requirements**: cargo test --all-targets must pass with zero failures
- **No external network, LLM, cloud dependencies**

---

## 15. PHASE 0 GATE SELF-ASSESSMENT

### Gate Checklist

| Check | Status | Notes |
|-------|--------|-------|
| Player fantasy is defined | PASS | Section 1 |
| Player role is defined | PASS | Section 2 |
| Core loop is defined | PASS | Section 3 |
| Minute-to-minute play is described | PASS | Section 4 |
| 30-minute experience is described | PASS | Section 5 |
| Multi-hour arc is described | PASS | Section 6 |
| Longitudinal arc is described | PASS | Section 7 |
| Progression model covers 6 dimensions | PASS | Section 8 |
| Consequence model is defined | PASS | Section 9 |
| Failure and recovery are defined | PASS | Section 10 |
| Fun hypotheses are falsifiable | PASS | Section 11 (6 hypotheses) |
| Scope exclusions are explicit | PASS | Section 12 |
| Acceptance criteria are observable | PASS | Section 13 (14 criteria) |
| Contract is internally consistent | PASS | No detected contradictions |
| Scope is bounded and finishable | PASS | Single settlement, 1 transformation, ~30 NPCs |
| No requirement eliminates another | PASS | Checked |

### Contradiction Check

- Determinism requirement vs. player input: **Resolved**. Player input is
  explicitly non-deterministic; NPC behavior is deterministic. The two are
  structurally separated.
- No-authored-quests vs. fun: **Resolved by FH-3**. Systemic goal generation
  must be verified, not assumed.
- Terminal UI vs. engagement: **Resolved by scope boundary**. VS1 validates
  the systems thesis, not the presentation thesis.
- CIVITAS-1M economy scale vs. ~30 NPC settlement: **Resolved**. CIVITAS-1M
  can operate at this scale; no scale conflict.

---

## PHASE 0 GATE DISPOSITION

`GODSEED_PHASE_0_PASS`

The contract is internally coherent, bounded, and ready for Phase 1 Architecture.

No frozen requirement has been eliminated or softened.

No feasibility concern prevents Phase 1 initiation.

---

*Contract frozen at: 2026-09-29T07:30:00-07:00*
*Executor attestation: This contract was not modified after initial composition to
satisfy implementation convenience. All requirements are architectural challenges
to be solved, not negotiated away.*
