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
struct BracketLib {
    bterm: BTerm,
}

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

fn cls(mut bl: ResMut<BracketLib>) {
    bl.bterm.cls();
}

fn draw_things(mut bl: ResMut<BracketLib>, query: Query<(&Position, &Renderable)>) {
    for (p, r) in &query {
        bl.bterm.set(p.x, p.y, r.fg, r.bg, r.glyph);
    }
}

fn move_left(mut query: Query<&mut Position, With<LeftMover>>) {
    for mut p in query.iter_mut() {
        p.x -= 1;
    }
}

// Handle input that affects the player's position
fn player_input_move(
    mut bl: ResMut<BracketLib>,
    map: Res<Map>,
    mut query: Query<&mut Position, With<Player>>,
) {
    // Player movement
    let mut dx = 0;
    let mut dy = 0;
    match bl.bterm.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => dx -= 1,
            VirtualKeyCode::Right => dx += 1,
            VirtualKeyCode::Up => dy -= 1,
            VirtualKeyCode::Down => dy += 1,
            VirtualKeyCode::Q => bl.bterm.quit(),
            _ => {}
        },
    }

    for mut p in query.iter_mut() {
        if map.terrain[xy_idx(p.x + dx, p.y + dy)] != TerrainType::Water {
            p.x += dx;
            p.y += dy;
        }
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

// map stuff

#[derive(Resource)]
struct Map {
    terrain: Vec<TerrainType>,
}

#[derive(PartialEq, Copy, Clone)]
enum TerrainType {
    Land,
    Water,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

fn new_map() -> Vec<TerrainType> {
    let mut map = vec![TerrainType::Land; 80 * 50];

    // Make the boundaries water
    for x in 0..80 {
        map[xy_idx(x, 0)] = TerrainType::Water;
        map[xy_idx(x, 49)] = TerrainType::Water;
    }
    for y in 0..50 {
        map[xy_idx(0, y)] = TerrainType::Water;
        map[xy_idx(79, y)] = TerrainType::Water;
    }

    // Now we'll randomly splat a bunch of water. It won't be pretty, but it's a decent illustration.
    // First, obtain the thread-local RNG:
    let mut rng = RandomNumberGenerator::new();

    for _ in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 30) {
            map[idx] = TerrainType::Water;
        }
    }

    map
}

fn draw_map(mut bl: ResMut<BracketLib>, map: Res<Map>) {
    let mut y = 0;
    let mut x = 0;
    for terrain in map.terrain.iter() {
        // Render a tile depending upon the tile type
        match terrain {
            TerrainType::Land => {
                bl.bterm.set(
                    x,
                    y,
                    RGB::named(terminal::GRAY),
                    RGB::named(terminal::BLACK),
                    to_cp437('.'),
                );
            }
            TerrainType::Water => {
                bl.bterm.set(
                    x,
                    y,
                    RGB::named(terminal::BLUE),
                    RGB::named(terminal::BLACK),
                    to_cp437('~'),
                );
            }
        }

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

// game loop stuff

// bracketlib's main loop expects a GameState with a tick function to call each frame/tick/update
struct State {
    app: App,
}
impl GameState for State {
    fn tick(&mut self, bterm: &mut BTerm) {
        // Reference lifetime problems arise if trying to put a reference to ctx into Context
        // Workaround is to clone from ctx into Context, tick, then clone back.
        // ctx is stale between the clones here, and Context is stale outside of this function,
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
    let gs = State { app };
    let _ = main_loop(bterm, gs);
}

fn main() {
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
