use crate::{BracketLib, Position, Renderable};
use bevy::prelude::*;

/// Clear the screen
pub fn cls(mut bl: ResMut<BracketLib>) {
    bl.bterm.cls();
}

/// Draw every renderable entity with a position onto the screen
pub fn draw_things(mut bl: ResMut<BracketLib>, query: Query<(&Position, &Renderable)>) {
    for (p, r) in &query {
        bl.bterm.set(p.x, p.y, r.fg, r.bg, r.glyph);
    }
}
