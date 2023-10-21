use bevy::prelude::*;
use bracket_lib::prelude::*;

/// The sole user-controllable entity
#[derive(Component, Debug)]
pub struct Cursor;

/// x and y coordinates of an entity on the game map
#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// Marks NPCs as wanting to move left
// TODO codify that this component should only exist on entities with Position component
#[derive(Component)]
pub struct LeftMover;

/// Necessary info to draw an entity to the screen
#[derive(Component, Default)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}
