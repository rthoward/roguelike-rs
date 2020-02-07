use tcod::colors;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode;
use tcod::map::{FovAlgorithm, Map as FovMap};

use specs::{Entities, Entity, ReadStorage, System, World, WorldExt, WriteStorage};
use super::other::*;
use super::movement::*;
use crate::coord::Coord;
use crate::game_state::GameState;


pub enum InputAction {
    Move(i32, i32),
    Wait,
    Nothing,
}

pub struct TcodSystem {
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

        // Loop through keypresses until a player action is taken or we quit the game
        loop {
            let key = root.wait_for_keypress(true);
            let input_action: InputAction = match key {
                Key { printable: 'k', .. } => InputAction::Move(0, -1),
                Key { printable: 'j', .. } => InputAction::Move(0, 1),
                Key { printable: 'h', .. } => InputAction::Move(-1, 0),
                Key { printable: 'l', .. } => InputAction::Move(1, 0),
                Key { printable: 'y', .. } => InputAction::Move(-1, -1),
                Key { printable: 'u', .. } => InputAction::Move(1, -1),
                Key { printable: 'n', .. } => InputAction::Move(-1, 1),
                Key { printable: 'm', .. } => InputAction::Move(1, 1),
                Key { printable: '.', .. } => InputAction::Wait,
                _ => InputAction::Nothing,
            };

            if let InputAction::Move(x, y) = input_action {
                for (_player, movements) in (&players, &mut movements).join() {
                    movements.events.push(MoveEvent {
                        coord: Coord::new(x, y),
                    });
                }
                break;
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
            if !game_state.running { break }
        }
    }
}
