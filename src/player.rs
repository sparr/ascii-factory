use bevy::prelude::*;
use bracket_lib::prelude::*;
use bracket_lib::terminal;

use crate::bevy_bracket::BevyBracket;
use crate::components::{LeftMover, Player, Position, Renderable};
use crate::map::{xy_idx, Map, TerrainType};

/// Create the Player entity with other necessary components
pub fn add_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Position { x: 40, y: 30 },
        Renderable {
            glyph: terminal::to_cp437('@'),
            fg: RGB::named(terminal::YELLOW),
            bg: RGB::named(terminal::BLACK),
        },
    ));
}

/// Create multiple non-Player entities with other necessary components
pub fn add_npcs(mut commands: Commands) {
    for i in 0..8 {
        commands.spawn((
            Position {
                x: i * 10 + 5,
                y: 25,
            },
            Renderable {
                glyph: terminal::to_cp437('☺'),
                fg: RGB::named(terminal::RED),
                bg: RGB::named(terminal::BLACK),
            },
            LeftMover,
        ));
    }
}

/// Moves LeftMover entities to the left, randomly pausing
pub fn move_left(mut query: Query<&mut Position, With<LeftMover>>) {
    for mut p in query.iter_mut() {
        let mut rng = RandomNumberGenerator::new();
        if rng.roll_dice(1, 10) == 1 {
            p.x -= 1;
        }
    }
}

/// Handle input that affects the player's position
pub fn player_input_move(
    mut bl: ResMut<BevyBracket>,
    map: Res<Map>,
    mut query: Query<&mut Position, With<Player>>,
) {
    // Player movement
    let mut dx = 0;
    let mut dy = 0;
    match bl.bterm.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => dx -= 1,
            VirtualKeyCode::Right => dx += 1,
            VirtualKeyCode::Up => dy -= 1,
            VirtualKeyCode::Down => dy += 1,
            VirtualKeyCode::Q => bl.bterm.quit(),
            _ => {}
        },
    }

    for mut p in query.iter_mut() {
        if map.terrain[xy_idx(p.x + dx, p.y + dy)] != TerrainType::Water {
            p.x += dx;
            p.y += dy;
        }
    }
}

/// Update the position of any entity that has left the map, wrapping around to the other side
pub fn wrap_position(mut query: Query<&mut Position>) {
    for mut p in query.iter_mut() {
        if p.x < 0 {
            p.x += 80
        }
        if p.x > 79 {
            p.x -= 80
        }
        if p.y < 0 {
            p.y += 50
        }
        if p.y > 49 {
            p.y -= 50
        }
    }
}
