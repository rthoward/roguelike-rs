#![feature(drain_filter)]

use std::collections::HashMap;
use tcod::colors;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode;
use tcod::map::{FovAlgorithm, Map as FovMap};

use specs::{Entities, Entity, ReadStorage, System, World, WorldExt, WriteStorage};

mod systems;
mod components;
mod coord;
mod factories;
mod map;

use coord::Coord;
use map::Map;
use systems::{register, setup_dispatcher, tcod::TcodSystem};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 80;
const LIMIT_FPS: i32 = 60;

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
    world = register(world);

    let game_state = GameState {
        maps: vec![Map::make_dungeon_map(
            SCREEN_WIDTH,
            SCREEN_WIDTH,
            &mut world,
        )],
        running: true,
        map: 0,
    };

    let player = factories::player(&mut world, game_state.get_map().start);

    world.insert(game_state);
    world.insert(player);

    let mut dispatcher = setup_dispatcher(world);

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
