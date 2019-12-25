use crate::coord::Coord;
use tcod::colors::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
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
        self.in_bounds(coord) && !self.get(coord).blocked
    }

    fn in_bounds(&self, coord: &Coord) -> bool {
        coord.x >= 0 && coord.x < self.width && coord.y >= 0 && coord.y < self.height
    }

    fn set(&mut self, tile: Tile, coord: &Coord) {
        let index = self.index(coord);
        if index <= self.vec_size() {
            self.tiles.insert(index, tile);
        }
    }

    pub fn get(&self, coord: &Coord) -> Tile {
        self.tiles[self.index(coord)]
    }

    fn vec_size(&self) -> usize {
        (self.height * self.width) as usize
    }

    fn index(&self, coord: &Coord) -> usize {
        (self.height * coord.y + coord.x) as usize
    }
}

pub fn make_map(height: i32, width: i32) -> Map {
    let size = (height * width) as usize;
    let tiles = vec![Tile::empty(); size];
    let mut map = Map {
        height,
        width,
        tiles,
    };

    map.set(Tile::wall(), &Coord { x: 30, y: 22 });
    map.set(Tile::wall(), &Coord { x: 50, y: 22 });

    map
}

mod tests {
    use super::*;

    #[test]
    fn test_make_map() -> Result<(), String> {
        let map = make_map(10, 10);
        assert_eq!(100 as usize, map.tiles.len());
        Ok(())
    }

    #[test]
    fn test_map_set() -> Result<(), String> {
        let mut map = make_map(5, 5);
        let coord = Coord::new(1, 1);
        map.set(Tile::wall(), &coord);
        assert_eq!(&Tile::wall(), map.tiles.get(6).unwrap());
        Ok(())
    }

    #[test]
    fn test_map_get() -> Result<(), String> {
        let mut map = make_map(5, 5);
        let coord = Coord::new(1, 1);
        map.set(Tile::wall(), &coord);
        assert_eq!(Tile::wall(), map.get(&coord));
        Ok(())
    }

    #[test]
    fn test_can_move() -> Result<(), String> {
        let mut map = make_map(5, 5);
        let coord = Coord::new(1, 1);
        map.set(Tile::wall(), &coord);

        assert_eq!(false, map.can_move(&Coord::new(-1, 0)));
        assert_eq!(false, map.can_move(&Coord::new(0, -1)));
        assert_eq!(false, map.can_move(&Coord::new(-1, -1)));
        assert_eq!(false, map.can_move(&Coord::new(5, 5)));
        assert_eq!(false, map.can_move(&Coord::new(1, 1)));
        assert_eq!(true, map.can_move(&Coord::new(0, 0)));

        Ok(())
    }
}
