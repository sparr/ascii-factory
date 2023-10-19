// Some implementation follows these two books:
// https://bevyengine.org/learn/book/introduction/
// https://bfnightly.bracketproductions.com/rustbook/

use bevy::prelude::*;
use bracket_lib::prelude::*;
use bracket_lib::terminal;
use terminal::{FontCharType, GameState, RGB};

// https://bevyengine.org/learn/book/getting-started/resources/ suggests using a Resource
// to allow the ECS access to globally unique data such as a renderer
#[derive(Resource)]
struct Context(BTerm);

// There should be just one player, for now
#[derive(Component, Debug)]
struct Player;

// Entities with Position have an x and y coordinate in the world
#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

// TODO codify that this component should only exist on entities with Position component
#[derive(Component)]
struct LeftMover;

// Entities with Renderable can be drawn to the screen
#[derive(Component)]
struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

fn add_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Position { x: 40, y: 30 },
        Renderable {
            glyph: terminal::to_cp437('@'),
            fg: RGB::named(terminal::YELLOW),
            bg: RGB::named(terminal::BLACK),
        },
    ));
}

fn add_npcs(mut commands: Commands) {
    for i in 0..8 {
        commands.spawn((
            Position {
                x: i * 10 + 5,
                y: 25,
            },
            Renderable {
                glyph: terminal::to_cp437('☺'),
                fg: RGB::named(terminal::RED),
                bg: RGB::named(terminal::BLACK),
            },
            LeftMover,
        ));
    }
}

fn draw_things(mut context: ResMut<Context>, query: Query<(&Position, &Renderable)>) {
    context.0.cls();
    for (p, r) in &query {
        context.0.set(p.x, p.y, r.fg, r.bg, r.glyph);
    }
}

fn move_left(mut query: Query<&mut Position, With<LeftMover>>) {
    for mut p in query.iter_mut() {
        p.x -= 1;
    }
}

// Handle input that affects the player's position
fn player_input_move(context: ResMut<Context>, mut query: Query<&mut Position, With<Player>>) {
    // Player movement
    let mut dx = 0;
    let mut dy = 0;
    match context.0.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => dx -= 1,
            VirtualKeyCode::Right => dx += 1,
            VirtualKeyCode::Up => dy -= 1,
            VirtualKeyCode::Down => dy += 1,
            _ => {}
        },
    }

    for mut p in query.iter_mut() {
        p.x += dx;
        p.y += dy;
    }
}

fn wrap_position(mut query: Query<&mut Position>) {
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

// bracketlib's main loop expects a GameState with a tick function to call each frame/tick/update
struct State {
    app: App,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Reference lifetime problems arise if trying to put a reference to ctx into Context
        // Workaround is to clone from ctx into Context, tick, then clone back.
        // ctx is stale between the clones here, and Context is stale outside of this function,
        // but neither is used while stale so that's ok (for now).
        // TODO: Find a better way to handle this.
        self.app.world.resource_mut::<Context>().0.clone_from(ctx);
        self.app.update();
        ctx.clone_from(&self.app.world.resource_mut::<Context>().0)
    }
}

fn bracketlib_runner(mut app: App) {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        // uncomment these for benchmarking, to avoid idle time between frames
        // .with_vsync(false)
        // .with_fps_cap(9999.0)
        .build()
        .unwrap();
    app.insert_resource(Context(context.clone()));
    let gs = State { app };
    let _ = main_loop(context, gs);
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, (add_player, add_npcs))
        .add_systems(
            Update,
            (player_input_move, move_left, wrap_position, draw_things),
        )
        .set_runner(bracketlib_runner)
        .run();
}
