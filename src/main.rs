#![feature(drain_filter)]

use std::collections::HashMap;
use tcod::colors;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode;
use tcod::map::{FovAlgorithm, Map as FovMap};

use specs::{Entities, Entity, ReadStorage, System, World, WorldExt, WriteStorage};

mod components;
mod coord;
mod factories;
mod map;

use components::{
    CollisionComponent, CollisionEvent, MoveEvent, MovementComponent, PlayerComponent,
    PositionComponent, RenderComponent,
};
use coord::Coord;
use map::Map;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 80;
const LIMIT_FPS: i32 = 60;

struct GameState {
    running: bool,
    maps: Vec<Map>,
    map: usize,
}

impl GameState {
    fn get_map(&self) -> &Map {
        self.maps.get(self.map).expect("Could not get current map")
    }

    fn get_map_mut(&mut self) -> &mut Map {
        self.maps
            .get_mut(self.map)
            .expect("Could not get current map")
    }
}

#[derive(Default)]
struct MovementSystem {
    occupied_coords: HashMap<Coord, Entity>,
}
impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, PositionComponent>,
        WriteStorage<'a, MovementComponent>,
        WriteStorage<'a, CollisionComponent>,
        specs::ReadExpect<'a, GameState>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (mut positions, mut movements, mut collisions, game_state, entities) = data;
        let map = game_state.get_map();

        for (position, entity) in (&positions, &*entities).join() {
            self.occupied_coords.insert(position.coord, entity);
        }
        for (position, movements, entity) in (&mut positions, &mut movements, &*entities).join() {
            for MoveEvent { coord } in movements.events.iter() {
                let new_coord = position.coord.add(coord);
                let map_collision = !map.can_move(&new_coord);
                let entity_collision = if let Some(collidee) = self.occupied_coords.get(&new_coord)
                {
                    if let Some(collidable_collidee) = collisions.get_mut(*collidee) {
                        collidable_collidee.events.push(CollisionEvent {
                            collider: entity,
                            collidee: *collidee,
                        });
                        true
                    } else {
                        false
                    }
                } else {
                    false
                };
                if !map_collision && !entity_collision {
                    self.occupied_coords.remove(&position.coord);
                    position.coord = new_coord;
                    self.occupied_coords.insert(new_coord, entity);
                }
            }
        }
    }
}

struct EventDrain;
impl<'a> System<'a> for EventDrain {
    type SystemData = (
        WriteStorage<'a, MovementComponent>,
        WriteStorage<'a, CollisionComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (mut movement, mut collision) = data;

        for movement in (&mut movement).join() {
            for event in movement.events.drain(..) {
                dbg!(event);
            }
        }

        for collision in (&mut collision).join() {
            for event in collision.events.drain(..) {
                dbg!(event);
            }
        }
    }
}

struct TcodSystem {
    root: Root,
    console: Offscreen,
    fov_maps: Vec<FovMap>,
}
impl<'a> System<'a> for TcodSystem {
    type SystemData = (
        ReadStorage<'a, RenderComponent>,
        ReadStorage<'a, PositionComponent>,
        ReadStorage<'a, PlayerComponent>,
        WriteStorage<'a, MovementComponent>,
        specs::WriteExpect<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let root = &mut self.root;
        let console = &mut self.console;
        let (sprites, positions, players, mut movements, mut game_state) = data;
        let current_map = game_state.map;
        let map = game_state.get_map_mut();

        match self.fov_maps.get(current_map) {
            None => {
                let mut new_fov_map = FovMap::new(map.width, map.height);
                for y in 0..map.height {
                    for x in 0..map.width {
                        let tile = map.get(&Coord::new(x, y));
                        new_fov_map.set(x, y, !tile.block_sight, !tile.blocked);
                    }
                }
                self.fov_maps.insert(current_map, new_fov_map);
            }
            _ => {}
        }
        let fov_map = self
            .fov_maps
            .get_mut(current_map)
            .expect("Could not get fov map");

        let fov_algo = FovAlgorithm::Basic;
        let fov_light_walls = true;
        let torch_raidus = 10;

        for (_, pos) in (&players, &positions).join() {
            fov_map.compute_fov(
                pos.coord.x,
                pos.coord.y,
                torch_raidus,
                fov_light_walls,
                fov_algo,
            );
        }

        root.clear();
        console.clear();
        for y in 0..map.height {
            for x in 0..map.width {
                let coord = Coord::new(x, y);
                let tile = map.get(&coord);
                let visible = fov_map.is_in_fov(x, y);
                let bg_color = if visible { tile.bg_lit } else { tile.bg_dark };

                if visible {
                    map.explore(&coord);
                }

                if visible || tile.explored {
                    console.set_char_background(x, y, bg_color, BackgroundFlag::Set);
                }
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
            Key { printable: 'n', .. } => Some((-1, 1)),
            Key { printable: 'm', .. } => Some((1, 1)),
            _ => None,
        };

        if let Some((x, y)) = movement {
            for (_player, movements) in (&players, &mut movements).join() {
                movements.events.push(MoveEvent {
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
    tcod::system::set_fps(LIMIT_FPS);
    let tcod = TcodSystem {
        root,
        console,
        fov_maps: vec![],
    };

    let mut world = World::new();
    world.register::<PositionComponent>();
    world.register::<RenderComponent>();
    let mut dispatcher = specs::DispatcherBuilder::new()
        .with_thread_local(tcod)
        .with(MovementSystem::default(), "movement_system", &[])
        .with(EventDrain {}, "event_drain", &[])
        .build();

    dispatcher.setup(&mut world);

    let game_state = GameState {
        maps: vec![Map::make_dungeon_map(
            SCREEN_WIDTH,
            SCREEN_WIDTH,
            &mut world,
        )],
        running: true,
        map: 0,
    };

    factories::player(&mut world, game_state.get_map().start);

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
