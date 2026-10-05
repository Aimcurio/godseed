/// Godseed — Household management
use bevy_ecs::system::Resource;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::{CitizenId, HouseholdId, SettlementId};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Household {
    pub id: HouseholdId,
    pub settlement_id: SettlementId,
    pub head: CitizenId,
    pub members: Vec<CitizenId>,
    pub coins: f64,
    pub food_reserve: f64,
    pub home_location: Option<u16>, // LocationId.0
}

impl Household {
    pub fn new(id: HouseholdId, settlement: SettlementId, head: CitizenId) -> Self {
        Self {
            id,
            settlement_id: settlement,
            head,
            members: vec![head],
            coins: 50.0,
            food_reserve: 10.0,
            home_location: None,
        }
    }

    pub fn add_member(&mut self, member: CitizenId) {
        if !self.members.contains(&member) {
            self.members.push(member);
        }
    }

    pub fn remove_member(&mut self, member: CitizenId) {
        self.members.retain(|m| *m != member);
    }

    pub fn size(&self) -> usize {
        self.members.len()
    }
}

/// Resource holding all households
#[derive(Resource, Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct HouseholdDirectory {
    pub households: HashMap<u32, Household>, // HouseholdId.0 → Household
    pub next_id: u32,
}

impl HouseholdDirectory {
    pub fn new() -> Self {
        Self {
            households: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn get(&self, id: HouseholdId) -> Option<&Household> {
        self.households.get(&id.0)
    }

    pub fn get_mut(&mut self, id: HouseholdId) -> Option<&mut Household> {
        self.households.get_mut(&id.0)
    }

    pub fn insert(&mut self, hh: Household) {
        self.households.insert(hh.id.0, hh);
    }

    pub fn next_household_id(&mut self) -> HouseholdId {
        let id = HouseholdId(self.next_id);
        self.next_id += 1;
        id
    }
}
