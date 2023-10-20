use bevy::prelude::*;

use crate::bevy_bracket::BevyBracket;
use crate::components::{Position, Renderable};

/// Clear the screen
pub fn cls(mut bl: ResMut<BevyBracket>) {
    bl.bterm.cls();
}

/// Draw every renderable entity with a position onto the screen
pub fn draw_things(mut bl: ResMut<BevyBracket>, query: Query<(&Position, &Renderable)>) {
    for (p, r) in &query {
        bl.bterm.set(p.x, p.y, r.fg, r.bg, r.glyph);
    }
}
