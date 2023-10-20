use bevy::prelude::*;
use bracket_lib::prelude::*;

// https://bevyengine.org/learn/book/getting-started/resources/ suggests using a Resource
// to allow the ECS access to globally unique data such as a renderer
#[derive(Resource)]
pub struct BracketLib {
    pub bterm: BTerm,
}

// bracketlib's main loop expects a GameState with a tick function to call each frame/tick/update
pub struct BracketLibGameState {
    pub app: App,
}
impl GameState for BracketLibGameState {
    fn tick(&mut self, bterm: &mut BTerm) {
        // Reference lifetime problems arise if trying to put a reference to ctx into BracketLib Resource
        // Workaround is to clone from bterm into the resource, tick, then clone back.
        // bterm is stale between the clones here, and the resource is stale outside of this function,
        // but neither is used while stale so that's ok (for now).
        // TODO: Find a better way to handle this.
        self.app
            .world
            .resource_mut::<BracketLib>()
            .bterm
            .clone_from(bterm);
        self.app.update();
        bterm.clone_from(&self.app.world.resource_mut::<BracketLib>().bterm)
    }
}
