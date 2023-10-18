// Some implementation follows these two books:
// https://bevyengine.org/learn/book/introduction/
// https://bfnightly.bracketproductions.com/rustbook/

use bevy_ecs::prelude::*;
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
    for (p, r) in &query {
        context.0.set(p.x, p.y, r.fg, r.bg, r.glyph);
    }
}

fn move_left(mut query: Query<&mut Position, With<LeftMover>>) {
    for mut p in query.iter_mut() {
        p.x -= 1;
        if p.x < 0 {
            p.x = 79;
        }
    }
}

// bracketlib's main loop expects a GameState with a tick function to call each frame/tick/update
struct State {
    world: World, // the ECS world containing all of our entities, components, systems, schedules, resources, etc
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        // Reference lifetime problems arise if trying to put a reference to ctx into Global.context
        // Workaround is to clone from ctx into Global.context, tick, then clone back.
        // ctx is stale between the clones here, and the Context is stale outside of this function,
        // but neither is used while stale so that's ok (for now).
        // TODO: Find a better way to handle this.
        self.world.resource_mut::<Context>().0.clone_from(ctx);
        self.world.run_schedule(TickSchedule);
        ctx.clone_from(&self.world.resource_mut::<Context>().0)
    }
}

#[derive(bevy_ecs::schedule::ScheduleLabel, Hash, Debug, PartialEq, Eq, Clone)]
struct TickSchedule;

fn main() -> terminal::BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .with_vsync(false)
        .with_fps_cap(9999.0)
        .build()?;

    let mut gs = State {
        world: World::new(),
    };

    let mut init_schedule = Schedule::default();
    init_schedule.add_systems(add_player);
    init_schedule.add_systems(add_npcs);
    init_schedule.run(&mut gs.world);

    gs.world.insert_resource(Context(context.clone()));

    let mut tick_schedule = Schedule::default();
    tick_schedule.add_systems(move_left);
    tick_schedule.add_systems(draw_things);
    gs.world.add_schedule(tick_schedule, TickSchedule);

    main_loop(context, gs)
}
