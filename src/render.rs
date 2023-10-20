use crate::{BracketLib, Position, Renderable};
use bevy::prelude::*;

pub fn cls(mut bl: ResMut<BracketLib>) {
    bl.bterm.cls();
}

pub fn draw_things(mut bl: ResMut<BracketLib>, query: Query<(&Position, &Renderable)>) {
    for (p, r) in &query {
        bl.bterm.set(p.x, p.y, r.fg, r.bg, r.glyph);
    }
}
