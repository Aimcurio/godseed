/// Godseed — Settlement (Thornveil)
use bevy_ecs::system::Resource;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::{OccupationType, SettlementId};

/// Settlement state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settlement {
    pub id: SettlementId,
    pub name: String,
    pub population: u32,
    pub occupied_housing: u32,
    pub housing_capacity: u32,
    pub food_reserve: f64,
    pub resource_stockpile: HashMap<u8, f64>, // ResourceType ordinal → quantity
    pub market_prices: HashMap<u8, f32>,      // ResourceType ordinal → price
    pub job_headcounts: HashMap<OccupationType, u32>,
    pub daily_food_production: f32,
    pub daily_food_consumption: f32,
}

impl Settlement {
    pub fn new(id: SettlementId, name: String, population: u32, capacity: u32) -> Self {
        let mut market_prices = HashMap::new();
        // Initial prices
        market_prices.insert(0, 2.0f32); // Food
        market_prices.insert(1, 5.0f32); // Timber
        market_prices.insert(2, 8.0f32); // Stone
        market_prices.insert(3, 12.0f32); // Tools
        market_prices.insert(4, 20.0f32); // Luxury
        market_prices.insert(5, 6.0f32); // Herbs
        market_prices.insert(6, 15.0f32); // Ink
        market_prices.insert(7, 10.0f32); // Parchment

        let mut resource_stockpile = HashMap::new();
        resource_stockpile.insert(0, 200.0); // Initial food
        resource_stockpile.insert(1, 50.0); // Timber
        resource_stockpile.insert(2, 30.0); // Stone
        resource_stockpile.insert(3, 20.0); // Tools
        resource_stockpile.insert(5, 15.0); // Herbs
        resource_stockpile.insert(6, 5.0); // Ink
        resource_stockpile.insert(7, 10.0); // Parchment

        Self {
            id,
            name,
            population,
            occupied_housing: population,
            housing_capacity: capacity,
            food_reserve: 200.0,
            resource_stockpile,
            market_prices,
            job_headcounts: HashMap::new(),
            daily_food_production: 0.0,
            daily_food_consumption: 0.0,
        }
    }

    pub fn get_price(&self, resource_ordinal: u8) -> f32 {
        *self.market_prices.get(&resource_ordinal).unwrap_or(&10.0)
    }

    pub fn get_stock(&self, resource_ordinal: u8) -> f64 {
        *self
            .resource_stockpile
            .get(&resource_ordinal)
            .unwrap_or(&0.0)
    }

    pub fn add_stock(&mut self, resource_ordinal: u8, quantity: f64) {
        *self
            .resource_stockpile
            .entry(resource_ordinal)
            .or_insert(0.0) += quantity;
    }

    pub fn remove_stock(&mut self, resource_ordinal: u8, quantity: f64) -> bool {
        let current = self.get_stock(resource_ordinal);
        if current < quantity {
            return false;
        }
        *self
            .resource_stockpile
            .entry(resource_ordinal)
            .or_insert(0.0) -= quantity;
        true
    }

    /// Update market price based on stock levels (simple supply/demand)
    pub fn update_price(&mut self, resource_ordinal: u8, base_price: f32, demand_per_day: f32) {
        let stock = self.get_stock(resource_ordinal) as f32;
        let days_of_stock = if demand_per_day > 0.0 {
            stock / demand_per_day
        } else {
            30.0
        };
        // Price rises when stock < 7 days, falls when > 20 days
        let factor = if days_of_stock < 3.0 {
            2.5
        } else if days_of_stock < 7.0 {
            1.5
        } else if days_of_stock < 14.0 {
            1.0
        } else if days_of_stock < 20.0 {
            0.85
        } else {
            0.7
        };
        let new_price = (base_price * factor).max(0.5);
        let current = self
            .market_prices
            .entry(resource_ordinal)
            .or_insert(base_price);
        // Smooth price adjustment (80% old, 20% new per week)
        *current = (*current * 0.8 + new_price * 0.2).max(0.5);
    }
}

/// Resource holding all settlements (VS1: just one, Thornveil)
#[derive(Resource, Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SettlementDirectory {
    pub settlements: HashMap<u16, Settlement>, // SettlementId.0 → Settlement
}

impl SettlementDirectory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, id: SettlementId) -> Option<&Settlement> {
        self.settlements.get(&id.0)
    }

    pub fn get_mut(&mut self, id: SettlementId) -> Option<&mut Settlement> {
        self.settlements.get_mut(&id.0)
    }

    pub fn insert(&mut self, settlement: Settlement) {
        self.settlements.insert(settlement.id.0, settlement);
    }

    /// The Thornveil settlement ID
    pub fn thornveil_id() -> SettlementId {
        SettlementId(1)
    }
}
