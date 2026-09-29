/// Godseed — Main Simulation Engine
///
/// Orchestrates the Bevy ECS world, multirate schedule, and all systems.
/// Player input arrives via PlayerInputBuffer; all other state is deterministic.

use bevy_ecs::prelude::*;
use std::io;
use std::fs;

use crate::components::{
    CausalAudit, CapabilitySet, CitizenMeta, Demographics, Disposition, HouseholdRef,
    Inventory, Kinship, KnowledgeInventory, MobilityProfile, NpcGoals, NpcMemory, NpcSchedule,
    OccupationProfile, PersonalFinances, PhysicalNeeds, PlayerInputBuffer, PlayerMarker,
    SettlementRef, TransformationState,
};
use crate::content::ContentDefinitions;
use crate::household::HouseholdDirectory;
use crate::invariants::verify_invariants;
use crate::npc::spawn_thornveil_npcs;
use crate::persistence::{CitizenSnapshot, SimulationSnapshot, load_snapshot, save_snapshot};
use crate::player::spawn_player;
use crate::replay::compute_authoritative_state_hash;
use crate::resources::{
    EventRing, NextCausalId, NextCitizenId, PendingConsequenceRegistry, RelationshipLedger,
    ReputationRegistry, ReturnDigestLog, TelemetryLog,
};
use crate::settlement::{Settlement, SettlementDirectory};
use crate::systems::{
    consequence::pending_consequence_progression_system,
    demographics::demographics_aging_system,
    gossip::gossip_system,
    labor::labor_market_system,
    market::{household_consumption_system, market_price_update_system},
    npc_goal::npc_goal_system,
    npc_memory::npc_memory_system,
    npc_routine::npc_routine_system,
    physiology::physiology_system,
    player_action::player_action_system,
    production::production_system,
    relationship::relationship_decay_system,
    telemetry_sys::telemetry_system,
    transformation::transformation_check_system,
};
use crate::types::{
    ActionResult, CitizenId, LocationId, PlayerAction, SettlementId, SimClock,
};
use crate::world::WorldMap;


pub struct Simulation {
    pub world: World,
    daily_schedule: Schedule,
    weekly_schedule: Schedule,
    monthly_schedule: Schedule,
}

impl Simulation {
    /// Create a new Godseed simulation (Thornveil from scratch)
    pub fn new() -> Self {
        let mut world = World::new();

        // ── Resources ─────────────────────────────────────────────────────────
        world.insert_resource(SimClock::new());
        world.insert_resource(WorldMap::thornveil());
        world.insert_resource(SettlementDirectory::new());
        world.insert_resource(HouseholdDirectory::new());
        world.insert_resource(RelationshipLedger::default());
        world.insert_resource(ReputationRegistry::default());
        world.insert_resource(EventRing::new(10_000));
        world.insert_resource(TelemetryLog::new());
        world.insert_resource(NextCitizenId(100)); // NPCs use 1–99; new births start at 100
        world.insert_resource(ContentDefinitions::thornveil());
        world.insert_resource(PendingConsequenceRegistry::default());
        world.insert_resource(ReturnDigestLog::default());
        world.insert_resource(NextCausalId::default());

        // ── Initialize Thornveil settlement ───────────────────────────────────
        let thornveil = Settlement::new(
            SettlementId(1),
            "Thornveil".to_string(),
            0,   // population updated after spawning
            60,  // housing capacity
        );
        world.resource_mut::<SettlementDirectory>().insert(thornveil);

        // ── Spawn NPCs from content definitions ───────────────────────────────
        spawn_thornveil_npcs(&mut world);

        // ── Spawn player entity ───────────────────────────────────────────────
        spawn_player(&mut world);

        // ── Update settlement population count ────────────────────────────────
        let npc_count = {
            // Count all living citizens excluding player
            let mut all_q = world.query::<&CitizenMeta>();
            all_q.iter(&world).filter(|m| m.alive && m.id != CitizenId::PLAYER).count() as u32
        };

        if let Some(settlement) = world.resource_mut::<SettlementDirectory>().get_mut(SettlementId(1)) {
            settlement.population = npc_count;
        }

        let sim = Self {
            world,
            daily_schedule: Self::build_daily_schedule(),
            weekly_schedule: Self::build_weekly_schedule(),
            monthly_schedule: Self::build_monthly_schedule(),
        };

        sim
    }

    fn build_daily_schedule() -> Schedule {
        let mut s = Schedule::default();
        s.add_systems((
            npc_routine_system,
            player_action_system,
            production_system,
            physiology_system,
            npc_goal_system,
            npc_memory_system,
            pending_consequence_progression_system,
            demographics_aging_system,
            telemetry_system,
        ));
        s
    }

    fn build_weekly_schedule() -> Schedule {
        let mut s = Schedule::default();
        s.add_systems((
            market_price_update_system,
            household_consumption_system,
            labor_market_system,
            gossip_system,
            relationship_decay_system,
        ));
        s
    }

    fn build_monthly_schedule() -> Schedule {
        let mut s = Schedule::default();
        s.add_systems((
            transformation_check_system,
        ));
        s
    }

    /// Advance simulation by one tick
    pub fn step(&mut self) -> u64 {
        let tick = {
            let mut clock = self.world.resource_mut::<SimClock>();
            clock.tick += 1;
            clock.tick
        };

        self.daily_schedule.run(&mut self.world);

        if tick % 7 == 0 { // Every 7 ticks (simplified weekly for terminal play)
            self.weekly_schedule.run(&mut self.world);
        }
        if tick % 30 == 0 { // Every 30 ticks (simplified monthly)
            self.monthly_schedule.run(&mut self.world);
        }

        tick
    }

    /// Advance by N ticks (time skip)
    pub fn advance(&mut self, ticks: u64) {
        for _ in 0..ticks {
            self.step();
        }
    }

    /// Run the weekly schedule explicitly (for testing and situational triggers)
    pub fn step_weekly(&mut self) {
        self.weekly_schedule.run(&mut self.world);
    }

    /// Push a player action into the input buffer
    pub fn push_action(&mut self, action: PlayerAction) {
        let mut q = self.world.query_filtered::<&mut PlayerInputBuffer, With<PlayerMarker>>();
        if let Some(mut buf) = q.iter_mut(&mut self.world).next() {
            buf.push_action(action);
        }
    }

    /// Drain action results from the player's result buffer
    pub fn drain_results(&mut self) -> Vec<ActionResult> {
        let mut results = Vec::new();
        let mut q = self.world.query_filtered::<&mut PlayerInputBuffer, With<PlayerMarker>>();
        if let Some(mut buf) = q.iter_mut(&mut self.world).next() {
            while let Some(r) = buf.last_results.pop_front() {
                results.push(r);
            }
        }
        results
    }

    /// Get current tick
    pub fn tick(&self) -> u64 {
        self.world.resource::<SimClock>().tick
    }

    /// Get player's current location
    pub fn player_location(&mut self) -> LocationId {
        let mut q = self.world.query_filtered::<&SettlementRef, With<PlayerMarker>>();
        q.iter(&self.world).next()
            .map(|r| r.current_location)
            .unwrap_or(LocationId(9))
    }

    /// State hash for determinism/regression testing
    pub fn state_hash(&mut self) -> u64 {
        compute_authoritative_state_hash(&mut self.world)
    }

    /// Verify invariants
    pub fn check_invariants(&mut self) -> Result<(), Vec<String>> {
        verify_invariants(&mut self.world)
    }

    /// Living NPC count (excludes player)
    pub fn living_npc_count(&mut self) -> usize {
        let mut q = self.world.query::<(&CitizenMeta, Option<&PlayerMarker>)>();
        q.iter(&self.world)
            .filter(|(m, is_player)| m.alive && is_player.is_none())
            .count()
    }

    /// Collect simulation summary for display
    pub fn summary(&mut self) -> SimSummary {
        let tick = self.tick();
        let day = tick / 24;
        let hour = tick % 24;

        let player_data = {
            let mut q = self.world.query_filtered::<(
                &CitizenMeta, &Demographics, &PhysicalNeeds, &PersonalFinances, &SettlementRef,
                &CapabilitySet, &TransformationState, &KnowledgeInventory,
            ), With<PlayerMarker>>();
            q.iter(&self.world).next().map(|(meta, demo, needs, fin, sref, caps, transform, knowledge)| {
                PlayerSummary {
                    alive: meta.alive,
                    satiety: needs.satiety,
                    health: demo.health,
                    rest: needs.rest,
                    coins: fin.coins,
                    location: sref.current_location,
                    transformation_stage: transform.stage,
                    transformation_progress: transform.progress,
                    inscriptions: transform.inscriptions_completed,
                    capability_count: caps.capabilities.len(),
                    knowledge_count: knowledge.nodes.len(),
                }
            })
        };

        SimSummary {
            tick,
            day,
            hour,
            living_npcs: self.living_npc_count(),
            player: player_data,
        }
    }

    /// Save simulation to file
    pub fn save(&mut self, path: &str) -> io::Result<()> {
        let snapshot = self.build_snapshot();
        let mut file = fs::File::create(path)?;
        save_snapshot(&snapshot, &mut file)?;
        Ok(())
    }

    /// Load simulation from file
    pub fn load_from_file(path: &str) -> io::Result<Self> {
        let mut file = fs::File::open(path)?;
        let snapshot = load_snapshot(&mut file)?;
        Ok(Self::from_snapshot(snapshot))
    }

    pub fn build_snapshot(&mut self) -> SimulationSnapshot {
        let clock = *self.world.resource::<SimClock>();
        let world_map = self.world.resource::<WorldMap>().clone();
        let settlements = self.world.resource::<SettlementDirectory>().clone();
        let households = self.world.resource::<HouseholdDirectory>().clone();
        let relationships = self.world.resource::<RelationshipLedger>().clone();
        let reputation = self.world.resource::<ReputationRegistry>().clone();
        let events = self.world.resource::<EventRing>().clone();
        let next_id = self.world.resource::<NextCitizenId>().0;

        let mut citizens = Vec::new();

        // Collect all citizen snapshots
        {
            let mut q = self.world.query::<(
                (
                    &CitizenMeta, &Demographics, &HouseholdRef, &SettlementRef,
                    &OccupationProfile, &PersonalFinances, &PhysicalNeeds,
                    &MobilityProfile, &Kinship, &CausalAudit, &Inventory,
                ),
                (
                    Option<&NpcMemory>, Option<&NpcSchedule>, Option<&NpcGoals>,
                    Option<&Disposition>, Option<&PlayerMarker>,
                    Option<&CapabilitySet>, Option<&TransformationState>, Option<&KnowledgeInventory>,
                ),
            )>();

            for (
                (meta, demo, hh_ref, sref, occ, fin, needs, mob, kin, audit, inv),
                (npc_mem, npc_sched, npc_goals, disp, is_player, caps, transform, knowledge),
            ) in q.iter(&self.world) {
                citizens.push(CitizenSnapshot {
                    meta: meta.clone(),
                    demographics: demo.clone(),
                    household_ref: hh_ref.clone(),
                    settlement_ref: sref.clone(),
                    occupation: occ.clone(),
                    finances: fin.clone(),
                    needs: needs.clone(),
                    mobility: mob.clone(),
                    kinship: kin.clone(),
                    causal_audit: audit.clone(),
                    inventory: inv.clone(),
                    npc_memory: npc_mem.cloned(),
                    npc_schedule: npc_sched.cloned(),
                    npc_goals: npc_goals.cloned(),
                    disposition: disp.cloned(),
                    is_player: is_player.is_some(),
                    capabilities: caps.cloned(),
                    transformation: transform.cloned(),
                    knowledge: knowledge.cloned(),
                });
            }
        }

        citizens.sort_by_key(|c| c.meta.id.0);


        SimulationSnapshot {
            version: 1,
            clock,
            world_map,
            settlements,
            households,
            relationships,
            reputation,
            events,
            next_citizen_id: next_id,
            citizens,
            seed: 0, // VS1 is not seeded-only; authored content
        }
    }

    pub fn from_snapshot(snapshot: SimulationSnapshot) -> Self {
        let mut world = World::new();

        world.insert_resource(snapshot.clock);
        world.insert_resource(snapshot.world_map);
        world.insert_resource(snapshot.settlements);
        world.insert_resource(snapshot.households);
        world.insert_resource(snapshot.relationships);
        world.insert_resource(snapshot.reputation);
        world.insert_resource(snapshot.events);
        world.insert_resource(NextCitizenId(snapshot.next_citizen_id));
        world.insert_resource(TelemetryLog::new()); // telemetry is session-local
        world.insert_resource(ContentDefinitions::thornveil()); // content is always re-loaded

        for c in snapshot.citizens {
            let mut builder = world.spawn((
                c.meta,
                c.demographics,
                c.household_ref,
                c.settlement_ref,
                c.occupation,
                c.finances,
                c.needs,
                c.mobility,
                c.kinship,
                c.causal_audit,
                c.inventory,
            ));

            if let Some(mem) = c.npc_memory { builder.insert(mem); }
            if let Some(sched) = c.npc_schedule { builder.insert(sched); }
            if let Some(goals) = c.npc_goals { builder.insert(goals); }
            if let Some(disp) = c.disposition { builder.insert(disp); }

            if c.is_player {
                builder.insert(PlayerMarker);
                builder.insert(PlayerInputBuffer::new());
                if let Some(caps) = c.capabilities { builder.insert(caps); }
                if let Some(t) = c.transformation { builder.insert(t); }
                if let Some(k) = c.knowledge { builder.insert(k); }
            }
        }

        Self {
            world,
            daily_schedule: Self::build_daily_schedule(),
            weekly_schedule: Self::build_weekly_schedule(),
            monthly_schedule: Self::build_monthly_schedule(),
        }
    }
}

// ── Summary Types ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SimSummary {
    pub tick: u64,
    pub day: u64,
    pub hour: u64,
    pub living_npcs: usize,
    pub player: Option<PlayerSummary>,
}

#[derive(Debug, Clone)]
pub struct PlayerSummary {
    pub alive: bool,
    pub satiety: u8,
    pub health: u8,
    pub rest: u8,
    pub coins: f64,
    pub location: LocationId,
    pub transformation_stage: u8,
    pub transformation_progress: u16,
    pub inscriptions: u32,
    pub capability_count: usize,
    pub knowledge_count: usize,
}

