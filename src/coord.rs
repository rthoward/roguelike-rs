pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }

    pub fn add(&self, other: Coord) -> Self {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
