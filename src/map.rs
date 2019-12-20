use tcod::colors::Color;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
    pub bg: Color,
}

impl Tile {
    const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
    const COLOR_DARK_GROUND: Color = Color {
        r: 50,
        g: 50,
        b: 150,
    };

    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
            bg: Tile::COLOR_DARK_GROUND,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
            bg: Tile::COLOR_DARK_WALL,
        }
    }
}

pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub height: i32,
    pub width: i32,
}

pub fn make_map() -> Map {
    // fill map with "unblocked" tiles
    let mut tiles = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    tiles[30][22] = Tile::wall();
    tiles[50][22] = Tile::wall();

    Map {
        height: MAP_HEIGHT,
        width: MAP_WIDTH,
        tiles,
    }
}
