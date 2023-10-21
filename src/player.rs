use bevy::prelude::*;
use bracket_lib::prelude::*;
use bracket_lib::terminal;

use crate::bevy_bracket::BevyBracket;
use crate::components::{Cursor, Position};
use crate::visibility::Viewshed;

/// Create the Player entity with other necessary components
pub fn add_cursor(mut commands: Commands) {
    commands.spawn((
        Cursor,
        Position { x: 40, y: 30 },
        Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
        },
    ));
}

pub fn draw_cursor(
    mut bl: ResMut<BevyBracket>,
    cursor_position: Query<&mut Position, With<Cursor>>,
) {
    for p in &cursor_position {
        bl.bterm.set_bg(p.x, 0, RGB::named(terminal::YELLOW));
        bl.bterm.set_bg(p.x, 49, RGB::named(terminal::YELLOW));
        bl.bterm.set_bg(0, p.y, RGB::named(terminal::YELLOW));
        bl.bterm.set_bg(79, p.y, RGB::named(terminal::YELLOW));
        bl.bterm.set_bg(p.x, p.y, RGB::named(terminal::YELLOW));
    }
}

/// Handle input
pub fn handle_input(mut bl: ResMut<BevyBracket>, mut query: Query<&mut Position, With<Cursor>>) {
    // Cursor movement
    let mut dx = 0;
    let mut dy = 0;
    match bl.bterm.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => dx -= 1,

            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => dx += 1,

            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => dy -= 1,

            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => dy += 1,

            VirtualKeyCode::Numpad1 | VirtualKeyCode::B => {
                dx -= 1;
                dy += 1;
            }

            VirtualKeyCode::Numpad3 | VirtualKeyCode::N => {
                dx += 1;
                dy += 1;
            }

            VirtualKeyCode::Numpad7 | VirtualKeyCode::Y => {
                dx -= 1;
                dy -= 1;
            }

            VirtualKeyCode::Numpad9 | VirtualKeyCode::U => {
                dx += 1;
                dy -= 1;
            }

            VirtualKeyCode::Q => bl.bterm.quit(),
            _ => {}
        },
    }

    for mut p in query.iter_mut() {
        p.x += dx;
        p.y += dy;
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
