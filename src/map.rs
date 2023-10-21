use bevy::prelude::*;
use bracket_lib::prelude::*;
use bracket_lib::terminal::{self, Point};

use crate::BevyBracket;

/// Types of terrain that can exist on the game world map
#[derive(PartialEq, Copy, Clone)]
pub enum TerrainType {
    /// Build stuff here
    Land,
    /// Can't build, can see across
    Water,
    /// Blocks line of sight
    Mountain,
}

/// Information about the game world map
#[derive(Resource)]
pub struct Map {
    pub terrain: Vec<TerrainType>,
    pub width: i32,
    pub height: i32,
    pub terrain_revealed: Vec<bool>,
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.terrain[idx] == TerrainType::Mountain
    }
}

impl Map {
    pub fn set_terrain(&mut self, p: Point, t: TerrainType) {
        let idx = self.point2d_to_index(p);
        self.terrain[idx] = t;
    }

    /// Create a map, mostly land with scattered water
    pub fn new_map() -> Map {
        let mut map = Map {
            terrain: vec![TerrainType::Land; 80 * 50],
            width: 80,
            height: 50,
            terrain_revealed: vec![false; 80 * 50],
        };

        // Make the boundaries mountain
        for x in 0..80 {
            map.set_terrain(Point { x, y: 0 }, TerrainType::Mountain);
            map.set_terrain(
                Point {
                    x,
                    y: map.height - 1,
                },
                TerrainType::Mountain,
            );
        }
        for y in 0..50 {
            map.set_terrain(Point { x: 0, y }, TerrainType::Mountain);
            map.set_terrain(
                Point {
                    x: map.width - 1,
                    y,
                },
                TerrainType::Mountain,
            );
        }

        // Now we'll randomly splat a bunch of water and mountains.
        // First, obtain the thread-local RNG:
        let mut rng = RandomNumberGenerator::new();

        for _ in 0..800 {
            let x = rng.roll_dice(1, 78);
            let y = rng.roll_dice(1, 48);
            if rng.roll_dice(1, 3) == 1 {
                map.set_terrain(Point { x, y }, TerrainType::Water);
            } else {
                map.set_terrain(Point { x, y }, TerrainType::Mountain);
            }
        }

        map
    }
}

/// Draw the visible part of the map to the screen
pub fn draw_map(mut bl: ResMut<BevyBracket>, map: Res<Map>) {
    for (idx, terrain) in map.terrain.iter().enumerate() {
        if map.terrain_revealed[idx] {
            let p = map.index_to_point2d(idx);
            // Render a tile depending upon the tile type
            match terrain {
                TerrainType::Land => {
                    bl.bterm.set(
                        p.x,
                        p.y,
                        RGB::named(terminal::GRAY),
                        RGB::named(terminal::BLACK),
                        to_cp437('.'),
                    );
                }
                TerrainType::Water => {
                    bl.bterm.set(
                        p.x,
                        p.y,
                        RGB::named(terminal::BLUE),
                        RGB::named(terminal::BLUE),
                        to_cp437('~'),
                    );
                }
                TerrainType::Mountain => {
                    bl.bterm.set(
                        p.x,
                        p.y,
                        RGB::named(terminal::BLACK),
                        RGB::named(terminal::BROWN1),
                        to_cp437('M'),
                    );
                }
            }
        }
    }
}
