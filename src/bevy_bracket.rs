/// This module contains functionality gluing together parts of bevy's
/// engine with bracket-lib's game loop and interface functionality
use bevy::prelude::*;
use bracket_lib::prelude::*;

/// Resource allowing bevy ecs to interact with bracket-lib functionality
// https://bevyengine.org/learn/book/getting-started/resources/ suggests using a Resource
// to allow the ECS access to globally unique data such as a renderer
#[derive(Resource)]
pub struct BevyBracket {
    pub bterm: BTerm,
}

/// Used by bracket-lib while running its game loop
pub struct BracketGameState {
    /// Makes bevy and bevy ecs functionality available to bracket-lib's main_loop and tick
    pub app: App,
}
impl GameState for BracketGameState {
    /// Called once per frame by bracket-lib's main_loop
    fn tick(&mut self, bterm: &mut BTerm) {
        // Reference lifetime problems arise if trying to put a reference to ctx into BracketLib Resource
        // Workaround is to clone from bterm into the resource, tick, then clone back.
        // bterm is stale between the clones here, and the resource is stale outside of this function,
        // but neither is used while stale so that's ok (for now).
        // TODO: Find a better way to handle this.
        self.app
            .world
            .resource_mut::<BevyBracket>()
            .bterm
            .clone_from(bterm);
        self.app.update();
        bterm.clone_from(&self.app.world.resource_mut::<BevyBracket>().bterm)
    }
}
