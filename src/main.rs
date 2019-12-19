use tcod::colors;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 80;
const LIMIT_FPS: i32 = 60;

struct Tcod {
    root: Root,
    console: Offscreen,
}

fn handle_keys(tcod: &mut Tcod, player_x: &mut i32, player_y: &mut i32) -> bool {
    let key = tcod.root.wait_for_keypress(true);
    match key {
        Key { printable: 'k', .. } => *player_y -= 1,
        Key { printable: 'j', .. } => *player_y += 1,
        Key { printable: 'h', .. } => *player_x -= 1,
        Key { printable: 'l', .. } => *player_x += 1,
        Key { printable: 'y', .. } => {
            *player_x -= 1;
            *player_y -= 1;
        }
        Key { printable: 'u', .. } => {
            *player_x += 1;
            *player_y -= 1;
        }
        Key { printable: 'n', .. } => {
            *player_x -= 1;
            *player_y += 1;
        }
        Key { printable: 'm', .. } => {
            *player_x += 1;
            *player_y += 1;
        }

        Key {
            code: KeyCode::Enter,
            alt: true,
            ..
        } => {
            tcod.root.set_fullscreen(!tcod.root.is_fullscreen());
        }

        Key {
            code: KeyCode::Escape,
            ..
        } => return true,

        _ => {}
    }
    false
}

fn main() {
    let mut player_x = SCREEN_WIDTH / 2;
    let mut player_y = SCREEN_HEIGHT / 2;

    let console = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("roguelike")
        .init();

    let mut tcod = Tcod { root, console };

    tcod::system::set_fps(LIMIT_FPS);

    while !tcod.root.window_closed() {
        tcod.console.set_default_foreground(colors::WHITE);
        tcod.console.clear();
        tcod.console
            .put_char(player_x, player_y, '@', BackgroundFlag::None);

        blit(
            &tcod.console,
            (0, 0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut tcod.root,
            (0, 0),
            1.0,
            1.0,
        );
        tcod.root.flush();

        let exit = handle_keys(&mut tcod, &mut player_x, &mut player_y);
        if exit {
            break;
        }
    }
}
