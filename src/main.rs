#![feature(drain_filter)] // TODO: need this?

use tcod::colors;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode;

mod components;
mod generational_index;
mod map;

use components::{Component, PositionComponent, RenderComponent};
use generational_index::{GenerationalIndex, GenerationalIndexAllocator, GenerationalIndexArray};
use map::{make_map, Map};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 80;
const LIMIT_FPS: i32 = 60;

struct Tcod {
    root: Root,
    console: Offscreen,
}

enum Event {
    Move {
        entity: GenerationalIndex,
        x: i32,
        y: i32,
    },
    Collision {
        collider: GenerationalIndex,
        collidee: GenerationalIndex,
    },
}

struct GameState {
    tcod: Tcod,
    running: bool,
    allocator: GenerationalIndexAllocator,

    position_components: GenerationalIndexArray<PositionComponent>,
    render_components: GenerationalIndexArray<RenderComponent>,
    pending_events: Vec<Event>,

    entities: Vec<GenerationalIndex>,

    maps: Vec<Map>,

    player: GenerationalIndex,
}

fn create_player() -> Vec<Component> {
    let player_position_component = PositionComponent {
        x: SCREEN_WIDTH / 2,
        y: SCREEN_HEIGHT / 2,
        map: 0,
    };
    let player_render_component = RenderComponent {
        glyph: '@',
        fg: colors::WHITE,
        bg: None,
    };
    vec![
        Component::Position(player_position_component),
        Component::Render(player_render_component),
    ]
}

fn initial_state() -> GameState {
    let console = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("roguelike")
        .init();
    let maps = vec![make_map()];

    let tcod = Tcod { root, console };
    tcod::system::set_fps(LIMIT_FPS);

    let mut allocator = GenerationalIndexAllocator::new();
    let mut position_components = GenerationalIndexArray::new();
    let mut render_components = GenerationalIndexArray::new();
    let mut entities = vec![];

    let component_lists = vec![create_player()];
    for components in component_lists {
        let i = allocator.allocate();
        entities.push(i);
        for component in components {
            match component {
                Component::Position(c) => {
                    position_components.insert(i, c);
                }
                Component::Render(c) => {
                    render_components.insert(i, c);
                }
            }
        }
    }
    let player = entities.first().unwrap().clone();

    GameState {
        tcod,
        allocator,
        running: true,
        entities,
        position_components,
        render_components,
        pending_events: vec![],
        player,
        maps,
    }
}

fn render_system(game_state: &mut GameState) {
    game_state.tcod.console.clear();
    let player_p: &PositionComponent = game_state
        .position_components
        .get(game_state.player)
        .unwrap();
    let map = &game_state.maps[player_p.map];

    for y in 0..map.height {
        for x in 0..map.width {
            let tile = map.tiles[x as usize][y as usize];
            game_state
                .tcod
                .console
                .set_char_background(x, y, tile.bg, BackgroundFlag::Set);
        }
    }

    for (i, r) in game_state.render_components.iter() {
        let m = game_state.position_components.get(i).unwrap();

        game_state
            .tcod
            .console
            .put_char_ex(m.x, m.y, r.glyph, r.fg, r.bg.unwrap_or(colors::BLACK));

        blit(
            &game_state.tcod.console,
            (0, 0),
            (map.width, map.height),
            &mut game_state.tcod.root,
            (0, 0),
            1.0,
            1.0,
        );
    }
    game_state.tcod.root.flush();
}

fn input_system(game_state: &mut GameState) {
    let key = game_state.tcod.root.wait_for_keypress(true);

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

    match movement {
        Some((x, y)) => game_state.pending_events.push(Event::Move {
            entity: game_state.player,
            x,
            y,
        }),
        None => {}
    };

    match key {
        Key {
            code: KeyCode::Enter,
            alt: true,
            ..
        } => {
            game_state
                .tcod
                .root
                .set_fullscreen(!game_state.tcod.root.is_fullscreen());
        }

        Key { printable: 'q', .. } => game_state.running = false,

        _ => {}
    }
}

fn movement_system(game_state: &mut GameState) {
    let events = game_state.pending_events.drain_filter(|a| match a {
        Event::Move { .. } => true,
        _ => false,
    });
    for event in events {
        match event {
            Event::Move { entity, x, y } => {
                if let Some(pc) = game_state.position_components.get_mut(entity) {
                    let new_x = pc.x + x;
                    let new_y = pc.y + y;
                    if let Some(map) = game_state.maps.get(pc.map) {
                        if map.can_move(new_x, new_y) {
                            pc.x = new_x;
                            pc.y = new_y;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let mut game_state = initial_state();
    while game_state.running && !game_state.tcod.root.window_closed() {
        render_system(&mut game_state);
        input_system(&mut game_state);
        movement_system(&mut game_state);
    }
}
