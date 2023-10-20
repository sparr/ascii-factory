use bevy::prelude::{Res, ResMut, Resource};
use bracket_lib::prelude::{to_cp437, RandomNumberGenerator, RGB};
use bracket_lib::terminal;

use crate::BevyBracket;

/// Types of terrain that can exist on the game world map
#[derive(PartialEq, Copy, Clone)]
pub enum TerrainType {
    Land,
    Water,
}

/// Information about the game world map
#[derive(Resource)]
pub struct Map {
    pub terrain: Vec<TerrainType>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    /// Convert 2d coordinates to the equivalent index in a 1d vector
    pub fn xy_idx(&mut self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn set_terrain(&mut self, x: i32, y: i32, t: TerrainType) {
        let idx = self.xy_idx(x, y);
        self.terrain[idx] = t;
    }

    /// Create a map, mostly land with scattered water
    pub fn new_map() -> Map {
        let mut map = Map {
            terrain: vec![TerrainType::Land; 80 * 50],
            width: 80,
            height: 50,
        };

        // Make the boundaries water
        for x in 0..80 {
            map.set_terrain(x, 0, TerrainType::Water);
            map.set_terrain(x, map.height - 1, TerrainType::Water);
        }
        for y in 0..50 {
            map.set_terrain(0, y, TerrainType::Water);
            map.set_terrain(map.width - 1, y, TerrainType::Water);
        }

        // Now we'll randomly splat a bunch of water. It won't be pretty, but it's a decent illustration.
        // First, obtain the thread-local RNG:
        let mut rng = RandomNumberGenerator::new();

        for _ in 0..400 {
            let x = rng.roll_dice(1, 79);
            let y = rng.roll_dice(1, 49);
            map.set_terrain(x, y, TerrainType::Water);
        }

        map
    }
}

/// Draw the map to the screen
pub fn draw_map(mut bl: ResMut<BevyBracket>, map: Res<Map>) {
    let mut y = 0;
    let mut x = 0;
    for terrain in map.terrain.iter() {
        // Render a tile depending upon the tile type
        match terrain {
            TerrainType::Land => {
                bl.bterm.set(
                    x,
                    y,
                    RGB::named(terminal::GRAY),
                    RGB::named(terminal::BLACK),
                    to_cp437('.'),
                );
            }
            TerrainType::Water => {
                bl.bterm.set(
                    x,
                    y,
                    RGB::named(terminal::BLUE),
                    RGB::named(terminal::BLUE),
                    to_cp437('~'),
                );
            }
        }

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
