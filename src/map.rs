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

#[derive(Debug)]
pub struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }
}

pub struct Map {
    pub tiles: Vec<Tile>,
    pub height: i32,
    pub width: i32,
}

impl Map {
    pub fn make_empty_map(height: i32, width: i32) -> Self {
        let size = (height * width) as usize;
        let tiles = vec![Tile::wall(); size];
        Map {
            height,
            width,
            tiles,
        }
    }

    pub fn make_dungeon_map(height: i32, width: i32) -> Self {
        let mut map = Self::make_empty_map(height, width);
        map.set(Tile::wall(), &Coord { x: 30, y: 22 });
        map.set(Tile::wall(), &Coord { x: 50, y: 22 });
        map.add_room(Rect::new(20, 15, 4, 4));
        map
    }

    pub fn add_room(&mut self, room: Rect) {
        for x in (room.x1 + 1)..room.x2 {
            for y in (room.y1 + 1)..room.y2 {
                self.set(Tile::empty(), &Coord::new(x, y));
            }
        }
    }

    pub fn can_move(&self, coord: &Coord) -> bool {
        self.in_bounds(coord) && !self.get(coord).blocked
    }

    pub fn get(&self, coord: &Coord) -> Tile {
        self.tiles[self.index(coord)]
    }

    fn in_bounds(&self, coord: &Coord) -> bool {
        coord.x >= 0 && coord.x < self.width && coord.y >= 0 && coord.y < self.height
    }

    fn set(&mut self, tile: Tile, coord: &Coord) {
        let index = self.index(coord);
        dbg!(index);
        if index <= self.vec_size() {
            self.tiles[index] = tile;
        }
    }

    fn vec_size(&self) -> usize {
        (self.height * self.width) as usize
    }

    fn index(&self, coord: &Coord) -> usize {
        (self.height * coord.y + coord.x) as usize
    }

    fn to_s(&self) -> String {
        let mut s = String::new();
        for x in 0..self.width {
            for y in 0..self.height {
                let tile = self.get(&Coord::new(x, y));
                let c = if tile.blocked { '#' } else { '.' };
                s.push(c);
            }
            s.push('\n')
        }
        s
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_make_map() -> Result<(), String> {
        let map = Map::make_empty_map(10, 10);
        assert_eq!(100 as usize, map.tiles.len());
        Ok(())
    }

    #[test]
    fn test_map_set() -> Result<(), String> {
        let mut map = Map::make_empty_map(5, 5);
        let coord = Coord::new(1, 1);
        map.set(Tile::wall(), &coord);
        assert_eq!(&Tile::wall(), map.tiles.get(6).unwrap());
        Ok(())
    }

    #[test]
    fn test_map_get() -> Result<(), String> {
        let mut map = Map::make_empty_map(5, 5);
        let coord = Coord::new(1, 1);
        map.set(Tile::wall(), &coord);
        assert_eq!(Tile::wall(), map.get(&coord));
        Ok(())
    }

    #[test]
    fn test_can_move() -> Result<(), String> {
        let mut map = Map::make_empty_map(5, 5);
        let coord = Coord::new(1, 1);
        map.set(Tile::empty(), &coord);

        assert_eq!(false, map.can_move(&Coord::new(-1, 0)));
        assert_eq!(false, map.can_move(&Coord::new(0, -1)));
        assert_eq!(false, map.can_move(&Coord::new(-1, -1)));
        assert_eq!(false, map.can_move(&Coord::new(5, 5)));

        assert_eq!(true, map.can_move(&Coord::new(1, 1)));

        Ok(())
    }

    #[test]
    fn test_add_room() -> Result<(), String> {
        /*
            Create a 3x3 room on a 5x5 map, starting in the top-left corner.

            #####
            #..##
            #..##
            #####
            #####
        */

        let mut map = Map::make_empty_map(5, 5);
        map.add_room(Rect::new(0, 0, 3, 3));

        assert_eq!(false, map.can_move(&Coord::new(0, 0)));
        assert_eq!(false, map.can_move(&Coord::new(0, 1)));
        assert_eq!(false, map.can_move(&Coord::new(0, 2)));
        assert_eq!(false, map.can_move(&Coord::new(0, 3)));
        assert_eq!(false, map.can_move(&Coord::new(0, 4)));

        assert_eq!(false, map.can_move(&Coord::new(1, 0)));
        assert_eq!(true, map.can_move(&Coord::new(1, 1)));
        assert_eq!(true, map.can_move(&Coord::new(1, 2)));
        assert_eq!(false, map.can_move(&Coord::new(1, 3)));
        assert_eq!(false, map.can_move(&Coord::new(1, 4)));

        assert_eq!(false, map.can_move(&Coord::new(2, 0)));
        assert_eq!(true, map.can_move(&Coord::new(2, 1)));
        assert_eq!(true, map.can_move(&Coord::new(2, 2)));
        assert_eq!(false, map.can_move(&Coord::new(2, 3)));
        assert_eq!(false, map.can_move(&Coord::new(2, 4)));

        Ok(())
    }
}
