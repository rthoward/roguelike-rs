use crate::coord::Coord;
use rand::Rng;
use std::cmp;
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

    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        // returns true if this rectangle intersects with another one
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}

pub struct Map {
    pub tiles: Vec<Tile>,
    pub height: i32,
    pub width: i32,
    pub start: Coord,
}

impl Map {
    pub fn make_empty_map(height: i32, width: i32) -> Self {
        let size = (height * width) as usize;
        let tiles = vec![Tile::wall(); size];
        Map {
            height,
            width,
            tiles,
            start: Coord::new(0, 0),
        }
    }

    pub fn make_dungeon_map(height: i32, width: i32) -> Self {
        /*
        Generate a dungeon style map by generating up to MAX_ROOMS non-overlapping rooms
        and connecting each one to its previous using either an vertical or horizontal
        tunnel.
        */

        let max_rooms = 10;
        let room_min_size = 6;
        let room_max_size = 10;

        let mut map = Self::make_empty_map(height, width);
        let mut rooms: Vec<Rect> = vec![];

        for _ in 0..max_rooms {
            let w = rand::thread_rng().gen_range(room_min_size, room_max_size + 1);
            let h = rand::thread_rng().gen_range(room_min_size, room_max_size + 1);
            let x = rand::thread_rng().gen_range(0, width - w);
            let y = rand::thread_rng().gen_range(0, height - h);
            let room = Rect::new(x, y, w, h);

            let failed = rooms
                .iter()
                .any(|other_room| room.intersects_with(other_room));
            if !failed {
                map.add_room(&room);
                let (new_x, new_y) = room.center();
                if rooms.is_empty() {
                    map.start = Coord::new(new_x, new_y);
                } else {
                    let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                    if rand::random() {
                        map.add_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.add_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.add_vertical_tunnel(prev_y, new_y, prev_x);
                        map.add_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }
                rooms.push(room)
            }
        }

        map
    }

    pub fn add_room(&mut self, room: &Rect) {
        for x in (room.x1 + 1)..room.x2 {
            for y in (room.y1 + 1)..room.y2 {
                self.set(Tile::empty(), &Coord::new(x, y));
            }
        }
    }

    fn add_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
            self.set(Tile::empty(), &Coord::new(x, y));
        }
    }

    fn add_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
            self.set(Tile::empty(), &Coord::new(x, y));
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
        map.add_room(&Rect::new(0, 0, 3, 3));

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
