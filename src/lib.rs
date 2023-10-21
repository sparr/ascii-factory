use bevy::prelude::*;
use bracket_lib::prelude::*;

mod map;
use map::{draw_map, Map};

mod bevy_bracket;
use bevy_bracket::{BevyBracket, BracketGameState};

mod components;

mod player;
use player::{add_cursor, draw_cursor, handle_input, wrap_position};

mod render;
use render::{cls, draw_things};

mod visibility;
use visibility::update_visibility;

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
    app.insert_resource(BevyBracket {
        bterm: bterm.clone(),
    });
    app.insert_resource(Map::new_map());
    let gs = BracketGameState { app };
    let _ = main_loop(bterm, gs);
}

/// Create the bevy App, set up plugins and systems, run the custom App runner
pub fn run() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, add_cursor)
        .add_systems(
            Update,
            (
                handle_input.before(wrap_position),
                wrap_position,
                cls.before(draw_map),
                update_visibility.before(draw_map),
                draw_map.before(draw_things),
                draw_things.before(draw_cursor),
                draw_cursor,
            ),
        )
        .set_runner(bracketlib_runner)
        .run();
}
