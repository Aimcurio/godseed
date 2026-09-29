/// Godseed — Player entity spawning

use bevy_ecs::prelude::*;

use crate::components::{
    CausalAudit, CapabilitySet, CitizenMeta, Demographics, HouseholdRef, Inventory,
    Kinship, KnowledgeInventory, MobilityProfile, OccupationProfile, PersonalFinances,
    PhysicalNeeds, PlayerInputBuffer, PlayerMarker, SettlementRef, TransformationState,
};
use crate::household::{Household, HouseholdDirectory};
use crate::types::{
    CitizenId, DecisionTrace, Gender, HouseholdRole, MigrationStatus,
    OccupationType, ReasonCode, SettlementId,
};
use crate::world::WorldMap;

/// Spawn the player entity into the ECS world
pub fn spawn_player(world: &mut World) -> Entity {
    let player_hh_id = {
        let mut hh_dir = world.resource_mut::<HouseholdDirectory>();
        let id = hh_dir.next_household_id();
        let hh = Household {
            id,
            settlement_id: SettlementId(1),
            head: CitizenId::PLAYER,
            members: vec![CitizenId::PLAYER],
            coins: 5.0,
            food_reserve: 3.0,
            home_location: None,
        };
        hh_dir.insert(hh);
        id
    };

    let start_location = WorldMap::arrival_location();

    let mut entity = world.spawn((
        PlayerMarker,
        CitizenMeta {
            id: CitizenId::PLAYER,
            name: "You".to_string(),
            gender: Gender::Male, // overridden at character creation in VS1
            alive: true,
        },
        Demographics {
            age_years: 25,
            age_ticks: 0,
            health: 100,
            fertility_timer: 0,
        },
        HouseholdRef {
            household_id: player_hh_id,
            role: HouseholdRole::Head,
        },
        SettlementRef {
            settlement_id: SettlementId(1),
            current_location: start_location,
        },
        OccupationProfile {
            occupation: OccupationType::Unemployed,
            skill_level: 1,
            experience: 0,
            productivity: 1.0,
        },
        PersonalFinances {
            coins: 5.0,
            last_income: 0.0,
            last_expense: 0.0,
            debt: 0.0,
        },
        PhysicalNeeds {
            satiety: 75,
            shelter: 60,
            rest: 80,
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
    ));

    entity.insert((
        CapabilitySet::new(),
        TransformationState::new(),
        KnowledgeInventory::new(),
        PlayerInputBuffer::new(),
        CausalAudit {
            trace: DecisionTrace::new(ReasonCode::PlayerArrival, 0, 0.0, 0.0, 0),
        },
    ));

    entity.id()
}

