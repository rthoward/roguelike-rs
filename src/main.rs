#![feature(drain_filter)]

use std::collections::HashMap;
use tcod::colors;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode;

use specs::{Builder, Entities, Entity, ReadStorage, System, World, WorldExt, WriteStorage};

mod components;
mod coord;
mod generational_index;
mod map;

use components::{Event, EventsComponent, PlayerComponent, PositionComponent, RenderComponent};
use coord::Coord;
use map::{make_map, Map};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 80;
const LIMIT_FPS: i32 = 60;

struct TcodSystem {
    root: Root,
    console: Offscreen,
}

struct GameState {
    running: bool,
    maps: Vec<Map>,
    map: usize,
}

struct MovementSystem;
impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, PositionComponent>,
        WriteStorage<'a, EventsComponent>,
        specs::ReadExpect<'a, GameState>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (mut positions, mut events, game_state, entities) = data;
        let map = game_state
            .maps
            .get(game_state.map)
            .expect("Could not get map");

        let mut occupied_coords: HashMap<Coord, Entity> = HashMap::new();
        for (position, entity) in (&positions, &*entities).join() {
            occupied_coords.insert(position.coord, entity);
        }
        for (position, events, entity) in (&mut positions, &mut events, &*entities).join() {
            let mut new_events: Vec<Event> = vec![];
            for e in events.queue.iter() {
                match e {
                    Event::Move { coord } => {
                        let new_coord = position.coord.add(coord);
                        let map_collision = !map.can_move(&new_coord);
                        let entity_collision =
                            if let Some(collidee) = occupied_coords.get(&new_coord) {
                                new_events.push(Event::Collision {
                                    collider: entity,
                                    collidee: *collidee,
                                });
                                true
                            } else {
                                false
                            };
                        if !map_collision && !entity_collision {
                            occupied_coords.remove(&position.coord);
                            position.coord = new_coord;
                            occupied_coords.insert(new_coord, entity);
                        }
                    }
                    _ => (),
                }
            }
            events.queue.extend(new_events.iter())
        }
    }
}

struct EventDrain;
impl<'a> System<'a> for EventDrain {
    type SystemData = WriteStorage<'a, EventsComponent>;

    fn run(&mut self, mut events: Self::SystemData) {
        use specs::Join;

        for events in (&mut events).join() {
            for event in events.queue.drain(..) {
                dbg!(event);
            }
        }
    }
}

impl<'a> System<'a> for TcodSystem {
    type SystemData = (
        ReadStorage<'a, RenderComponent>,
        ReadStorage<'a, PositionComponent>,
        ReadStorage<'a, PlayerComponent>,
        WriteStorage<'a, EventsComponent>,
        specs::WriteExpect<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let root = &mut self.root;
        let console = &mut self.console;
        let (sprites, positions, players, mut events, mut game_state) = data;
        let map = game_state
            .maps
            .get(game_state.map)
            .expect("Could not find map");

        root.clear();
        console.clear();
        for y in 0..map.height {
            for x in 0..map.width {
                let tile = map.get(&Coord::new(x, y));
                console.set_char_background(x, y, tile.bg, BackgroundFlag::Set);
            }
        }
        for (sprite, pos) in (&sprites, &positions).join() {
            console.put_char_ex(
                pos.coord.x,
                pos.coord.y,
                sprite.glyph,
                sprite.fg,
                sprite.bg.unwrap_or(colors::BLACK),
            );
        }
        blit(
            console,
            (0, 0),
            (map.width, map.height),
            root,
            (0, 0),
            1.0,
            1.0,
        );
        root.flush();

        let key = root.wait_for_keypress(true);

        let movement: Option<(i32, i32)> = match key {
            Key { printable: 'k', .. } => Some((0, -1)),
            Key { printable: 'j', .. } => Some((0, 1)),
            Key { printable: 'h', .. } => Some((-1, 0)),
            Key { printable: 'l', .. } => Some((1, 0)),
            Key { printable: 'y', .. } => Some((-1, -1)),
            Key { printable: 'u', .. } => Some((1, -1)),
            Key { printable: 'b', .. } => Some((-1, 1)),
            Key { printable: 'n', .. } => Some((1, 1)),
            _ => None,
        };

        if let Some((x, y)) = movement {
            for (_player, events) in (&players, &mut events).join() {
                events.queue.push(Event::Move {
                    coord: Coord::new(x, y),
                });
            }
        }

        match key {
            Key {
                code: KeyCode::Enter,
                alt: true,
                ..
            } => {
                root.set_fullscreen(!root.is_fullscreen());
            }

            Key { printable: 'q', .. } => game_state.running = false,

            _ => {}
        }
        game_state.running = game_state.running && !root.window_closed();
    }
}

fn main() {
    let console = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("roguelike")
        .init();
    let game_state = GameState {
        maps: vec![make_map(SCREEN_WIDTH, SCREEN_WIDTH)],
        running: true,
        map: 0,
    };

    tcod::system::set_fps(LIMIT_FPS);
    let tcod = TcodSystem { root, console };

    let mut world = World::new();
    world.register::<PositionComponent>();
    world.register::<RenderComponent>();

    let mut dispatcher = specs::DispatcherBuilder::new()
        .with_thread_local(tcod)
        .with(MovementSystem {}, "movement_system", &[])
        .with(EventDrain {}, "event_drain", &[])
        .build();

    dispatcher.setup(&mut world);

    world
        .create_entity()
        .with(PositionComponent {
            coord: Coord::new(1, 1),
            map: 0,
        })
        .with(RenderComponent {
            glyph: '@',
            fg: colors::WHITE,
            bg: None,
        })
        .with(EventsComponent { queue: vec![] })
        .with(PlayerComponent)
        .build();

    world
        .create_entity()
        .with(PositionComponent {
            coord: Coord::new(3, 3),
            map: 0,
        })
        .with(RenderComponent {
            glyph: 'o',
            fg: colors::GREEN,
            bg: None,
        })
        .build();

    world.insert(game_state);

    loop {
        dispatcher.dispatch(&mut world);
        {
            let game_state = world.read_resource::<GameState>();
            if !game_state.running {
                break;
            }
        }
        world.maintain();
    }
}
