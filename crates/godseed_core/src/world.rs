/// Godseed — World Map (Thornveil grid layout)
///
/// Thornveil is a hand-authored 20×20 grid with named locations.
/// The map is deterministic — no procedural generation needed for VS1.
use bevy_ecs::system::Resource;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::{CellType, LocationId, MapCell};

/// Named location definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    pub id: LocationId,
    pub name: String,
    pub description: String,
    pub location_type: LocationType,
    pub grid_x: u8,
    pub grid_y: u8,
    pub adjacent: Vec<LocationId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocationType {
    Inn,
    Forge,
    Market,
    Farmfield,
    Forest,
    HerbGarden,
    Archive,
    Well,
    StorageHouse,
    Residential,
    Road,
    OpenArea,
}

/// The Thornveil world map
#[derive(Resource, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldMap {
    pub width: u8,
    pub height: u8,
    pub cells: Vec<MapCell>,
    pub locations: HashMap<u16, Location>, // LocationId.0 → Location
}

impl WorldMap {
    /// Build the authored Thornveil map
    pub fn thornveil() -> Self {
        let width = 20u8;
        let height = 20u8;

        // Build a grid of cells
        let mut cells = Vec::with_capacity((width as usize) * (height as usize));
        for y in 0..height {
            for x in 0..width {
                let cell_type = Self::cell_type_at(x, y);
                let passable = !matches!(cell_type, CellType::Water);
                cells.push(MapCell {
                    x,
                    y,
                    cell_type,
                    passable,
                });
            }
        }

        let mut locations = HashMap::new();

        // Location IDs — canonical Thornveil locations
        let locs = Self::thornveil_locations();
        for loc in locs {
            locations.insert(loc.id.0, loc);
        }

        Self {
            width,
            height,
            cells,
            locations,
        }
    }

    fn cell_type_at(x: u8, y: u8) -> CellType {
        // Road through center
        if x == 10 {
            return CellType::Road;
        }
        if y == 10 {
            return CellType::Road;
        }
        // Water body (small stream, top-right)
        if x >= 15 && y <= 5 {
            return CellType::Water;
        }
        // Fields (south)
        if y >= 14 && x >= 2 && x <= 12 {
            return CellType::Field;
        }
        // Forest (west)
        if x <= 3 && y >= 4 && y <= 14 {
            return CellType::Forest;
        }
        // Buildings at key coordinates
        if (x == 9 || x == 10 || x == 11) && (y == 7 || y == 8 || y == 9) {
            return CellType::Building {
                location_id: LocationId(1),
            }; // Inn
        }
        if (x == 12 || x == 13) && (y == 8 || y == 9) {
            return CellType::Building {
                location_id: LocationId(2),
            }; // Forge
        }
        if (x == 9 || x == 10 || x == 11) && (y == 11 || y == 12) {
            return CellType::Building {
                location_id: LocationId(3),
            }; // Market
        }
        if (x == 5 || x == 6) && (y == 5 || y == 6) {
            return CellType::Building {
                location_id: LocationId(8),
            }; // Archive (ruined)
        }
        if x == 10 && y == 5 {
            return CellType::Building {
                location_id: LocationId(7),
            }; // Well
        }
        CellType::Open
    }

    pub fn thornveil_locations() -> Vec<Location> {
        vec![
            Location {
                id: LocationId(1),
                name: "The Slanted Timber (Inn)".to_string(),
                description: "A low-roofed inn with smoke-darkened beams. Smells of woodsmoke and stew. Mira Ashbridge keeps the place running.".to_string(),
                location_type: LocationType::Inn,
                grid_x: 10, grid_y: 8,
                adjacent: vec![LocationId(9), LocationId(3), LocationId(10)],
            },
            Location {
                id: LocationId(2),
                name: "Wren's Forge".to_string(),
                description: "A stone-walled workshop with a perpetually glowing hearth. The smell of hot iron hangs in the air.".to_string(),
                location_type: LocationType::Forge,
                grid_x: 12, grid_y: 8,
                adjacent: vec![LocationId(9), LocationId(3)],
            },
            Location {
                id: LocationId(3),
                name: "Market Square".to_string(),
                description: "A packed-earth square where the settlement trades. Stalls appear in the morning and pack up by dusk.".to_string(),
                location_type: LocationType::Market,
                grid_x: 10, grid_y: 11,
                adjacent: vec![LocationId(1), LocationId(2), LocationId(4), LocationId(5), LocationId(9)],
            },
            Location {
                id: LocationId(4),
                name: "North Fields".to_string(),
                description: "Long furrows of tilled earth. Oswin and others work here from dawn.".to_string(),
                location_type: LocationType::Farmfield,
                grid_x: 7, grid_y: 4,
                adjacent: vec![LocationId(3), LocationId(9)],
            },
            Location {
                id: LocationId(5),
                name: "South Fields".to_string(),
                description: "Wider and flatter than the north fields. Better yields in dry years.".to_string(),
                location_type: LocationType::Farmfield,
                grid_x: 7, grid_y: 15,
                adjacent: vec![LocationId(3), LocationId(6)],
            },
            Location {
                id: LocationId(6),
                name: "Herb Garden".to_string(),
                description: "A sheltered patch behind the south fields. Sera tends it at odd hours.".to_string(),
                location_type: LocationType::HerbGarden,
                grid_x: 7, grid_y: 17,
                adjacent: vec![LocationId(5)],
            },
            Location {
                id: LocationId(7),
                name: "The Well".to_string(),
                description: "Stone-rimmed, deep. People gather here in the morning to fill pails and exchange gossip.".to_string(),
                location_type: LocationType::Well,
                grid_x: 10, grid_y: 5,
                adjacent: vec![LocationId(4), LocationId(9), LocationId(8)],
            },
            Location {
                id: LocationId(8),
                name: "The Old Archive".to_string(),
                description: "A half-collapsed stone structure older than anyone in Thornveil. Rumored to have held records. Most of the shelves are rotted or missing.".to_string(),
                location_type: LocationType::Archive,
                grid_x: 5, grid_y: 5,
                adjacent: vec![LocationId(7), LocationId(11)],
            },
            Location {
                id: LocationId(9),
                name: "Settlement Road".to_string(),
                description: "The packed-mud main road through Thornveil. Everything connects to it.".to_string(),
                location_type: LocationType::Road,
                grid_x: 10, grid_y: 10,
                adjacent: vec![LocationId(1), LocationId(2), LocationId(3), LocationId(4), LocationId(7)],
            },
            Location {
                id: LocationId(10),
                name: "Storage House".to_string(),
                description: "A communal storage building. The lock is old but functional. The settlement elder keeps the key.".to_string(),
                location_type: LocationType::StorageHouse,
                grid_x: 12, grid_y: 12,
                adjacent: vec![LocationId(3), LocationId(9)],
            },
            Location {
                id: LocationId(11),
                name: "Forest Edge".to_string(),
                description: "Where the tree line begins. Timber is plentiful but the further in you go, the quieter it gets.".to_string(),
                location_type: LocationType::Forest,
                grid_x: 3, grid_y: 9,
                adjacent: vec![LocationId(8), LocationId(9)],
            },
        ]
    }

    pub fn get_location(&self, id: LocationId) -> Option<&Location> {
        self.locations.get(&id.0)
    }

    /// Check if two locations are adjacent (directly connected)
    pub fn are_adjacent(&self, from: LocationId, to: LocationId) -> bool {
        self.locations
            .get(&from.0)
            .map(|l| l.adjacent.contains(&to))
            .unwrap_or(false)
    }

    /// Starting location for new arrivals
    pub fn arrival_location() -> LocationId {
        LocationId(9) // Settlement Road
    }
}
