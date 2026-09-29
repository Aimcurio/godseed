/// Godseed — NPC spawning from content definitions

use bevy_ecs::prelude::*;

use crate::components::{
    CausalAudit, CitizenMeta, Demographics, Disposition, HouseholdRef, Inventory, Kinship,
    MobilityProfile, NpcGoals, NpcMemory, NpcSchedule, OccupationProfile, PersonalFinances,
    PhysicalNeeds, SettlementRef,
};
use crate::content::{ContentDefinitions, NpcDefinition};
use crate::household::{Household, HouseholdDirectory};
use crate::types::{
    CitizenId, DecisionTrace, HouseholdId, MigrationStatus, NpcActivity,
    ReasonCode, SettlementId,
};

/// Spawn all Thornveil NPCs from ContentDefinitions
pub fn spawn_thornveil_npcs(world: &mut World) {
    let definitions = {
        let content = world.resource::<ContentDefinitions>();
        content.npc_definitions.clone()
    };

    for def in &definitions {
        spawn_npc_from_def(world, def);
    }
}

fn spawn_npc_from_def(world: &mut World, def: &NpcDefinition) {
    // Ensure household exists
    let hh_id = HouseholdId(def.household_id);
    {
        let mut hh_dir = world.resource_mut::<HouseholdDirectory>();
        if hh_dir.get(hh_id).is_none() {
            let hh = Household {
                id: hh_id,
                settlement_id: SettlementId(1),
                head: CitizenId(def.citizen_id),
                members: vec![CitizenId(def.citizen_id)],
                coins: def.starting_coins,
                food_reserve: 10.0,
                home_location: Some(def.home_location.0 as u16),
            };
            hh_dir.insert(hh);
        } else {
            if let Some(hh) = hh_dir.get_mut(hh_id) {
                hh.add_member(CitizenId(def.citizen_id));
            }
        }
        // Update next_id
        if hh_dir.next_id <= def.household_id {
            hh_dir.next_id = def.household_id + 1;
        }
    }

    let mut disposition = Disposition::new(def.base_personality);
    disposition.will_teach = def.will_teach;
    disposition.teach_threshold = def.teach_threshold;

    let schedule = NpcSchedule {
        slots: def.schedule.clone(),
        current_activity: NpcActivity::Idle,
        home_location: def.home_location,
        work_location: def.work_location,
    };

    world.spawn((
        CitizenMeta {
            id: CitizenId(def.citizen_id),
            name: def.name.clone(),
            gender: def.gender,
            alive: true,
        },
        Demographics {
            age_years: def.age,
            age_ticks: 0,
            health: 100,
            fertility_timer: 0,
        },
        HouseholdRef {
            household_id: hh_id,
            role: def.household_role,
        },
        SettlementRef {
            settlement_id: SettlementId(1),
            current_location: def.home_location,
        },
        OccupationProfile {
            occupation: def.occupation,
            skill_level: 2,
            experience: 100,
            productivity: 1.2,
        },
        PersonalFinances {
            coins: def.starting_coins,
            last_income: 0.0,
            last_expense: 0.0,
            debt: 0.0,
        },
        PhysicalNeeds {
            satiety: 85,
            shelter: 90,
            rest: 90,
        },
        MobilityProfile {
            status: MigrationStatus::Settled,
        },
        Kinship {
            spouse: None,
            parent_a: None,
            parent_b: None,
            children_count: 0,
        },
        Inventory::new(),
        NpcMemory::new(),
        NpcGoals::new(),
        schedule,
        disposition,
        CausalAudit {
            trace: DecisionTrace::new(ReasonCode::InitialSpawn, 0, 0.0, 0.0, 0),
        },
    ));
}
