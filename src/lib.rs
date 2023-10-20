use bevy::prelude::*;
use bracket_lib::prelude::*;

mod map;
use map::*;

mod bracketlib;
use bracketlib::*;

mod components;
use components::*;

mod player;
use player::*;

mod render;
use render::*;

/// Used by bevy App.set_runner().run() to allow bracket-lib to control the game loop
fn bracketlib_runner(mut app: App) {
    let bterm = BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        // uncomment these for benchmarking, to avoid idle time between frames
        // .with_vsync(false)
        // .with_fps_cap(9999.0)
        .with_fullscreen(true)
        .build()
        .unwrap();
    app.insert_resource(BracketLib {
        bterm: bterm.clone(),
    });
    app.insert_resource(Map { terrain: new_map() });
    let gs = BracketLibGameState { app };
    let _ = main_loop(bterm, gs);
}

/// Create the bevy App, set up plugins and systems, run the custom App runner
pub fn run() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, (add_player, add_npcs))
        .add_systems(
            Update,
            (
                player_input_move.before(wrap_position),
                move_left.before(wrap_position),
                wrap_position,
                cls.before(draw_map),
                draw_map.before(draw_things),
                draw_things,
            ),
        )
        .set_runner(bracketlib_runner)
        .run();
}
