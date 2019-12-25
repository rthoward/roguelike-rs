use crate::coord::Coord;
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
    pub tiles: Vec<Tile>,
    pub height: i32,
    pub width: i32,
}

impl Map {
    pub fn can_move(&self, coord: &Coord) -> bool {
        !self.get(coord).blocked
    }

    fn set(&mut self, tile: Tile, coord: Coord) {
        let index = (coord.x * coord.y) as usize;
        if index <= self.vec_size() {
            self.tiles.insert(index, tile);
        }
    }

    pub fn get(&self, coord: &Coord) -> Tile {
        let index = (coord.x * coord.y) as usize;
        self.tiles[index]
    }

    fn vec_size(&self) -> usize {
        (self.height * self.width) as usize
    }
}

pub fn make_map() -> Map {
    let size = (MAP_HEIGHT * MAP_WIDTH) as usize;
    let tiles = vec![Tile::empty(); size];
    let mut map = Map {
        height: MAP_HEIGHT,
        width: MAP_WIDTH,
        tiles,
    };

    map.set(Tile::wall(), Coord { x: 30, y: 22 });
    map.set(Tile::wall(), Coord { x: 50, y: 22 });

    map
}
