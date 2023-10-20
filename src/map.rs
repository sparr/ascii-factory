use bevy::prelude::{Res, ResMut, Resource};
use bracket_lib::prelude::{to_cp437, RandomNumberGenerator, RGB};
use bracket_lib::terminal;

use crate::BracketLib;

#[derive(Resource)]
pub struct Map {
    pub terrain: Vec<TerrainType>,
}

#[derive(PartialEq, Copy, Clone)]
pub enum TerrainType {
    Land,
    Water,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn new_map() -> Vec<TerrainType> {
    let mut map = vec![TerrainType::Land; 80 * 50];

    // Make the boundaries water
    for x in 0..80 {
        map[xy_idx(x, 0)] = TerrainType::Water;
        map[xy_idx(x, 49)] = TerrainType::Water;
    }
    for y in 0..50 {
        map[xy_idx(0, y)] = TerrainType::Water;
        map[xy_idx(79, y)] = TerrainType::Water;
    }

    // Now we'll randomly splat a bunch of water. It won't be pretty, but it's a decent illustration.
    // First, obtain the thread-local RNG:
    let mut rng = RandomNumberGenerator::new();

    for _ in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 30) {
            map[idx] = TerrainType::Water;
        }
    }

    map
}

pub fn draw_map(mut bl: ResMut<BracketLib>, map: Res<Map>) {
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
