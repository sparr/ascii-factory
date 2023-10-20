use bevy::prelude::*;
use bracket_lib::prelude::*;

// There should be just one player, for now
#[derive(Component, Debug)]
pub struct Player;

// Entities with Position have an x and y coordinate in the world
#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

// TODO codify that this component should only exist on entities with Position component
#[derive(Component)]
pub struct LeftMover;

// Entities with Renderable can be drawn to the screen
#[derive(Component)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}
