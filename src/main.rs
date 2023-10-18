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

// Entities with Position have an x and y coordinate in the world
#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

// Entities with Renderable can be drawn to the screen
#[derive(Component)]
struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

fn add_player(mut commands: Commands) {
    commands.spawn((
        Position { x: 40, y: 30 },
        Renderable {
            glyph: terminal::to_cp437('@'),
            fg: RGB::named(terminal::YELLOW),
            bg: RGB::named(terminal::BLACK),
        },
    ));
}

fn add_npcs(mut commands: Commands) {
    for i in 0..11 {
        commands.spawn((
            Position {
                x: i * 7 + 5,
                y: 25,
            },
            Renderable {
                glyph: terminal::to_cp437('☺'),
                fg: RGB::named(terminal::RED),
                bg: RGB::named(terminal::BLACK),
            },
        ));
    }
}

fn draw_things(mut context: ResMut<Context>, query: Query<(&Position, &Renderable)>) {
    for (p, r) in &query {
        context.0.set(p.x, p.y, r.fg, r.bg, r.glyph);
    }
}

struct State {
    world: World, // the ECS world containing all of our entities, components, systems, schedules, resources, etc
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // reference lifetime problems arise if trying to put a reference to ctx into Global.context
        // workaround is to clone from ctx into Global.context, tick, then clone back
        // there's hopefully a better way to do this
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
    tick_schedule.add_systems(draw_things);
    gs.world.add_schedule(tick_schedule, TickSchedule);

    main_loop(context, gs)
}
