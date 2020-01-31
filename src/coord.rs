#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }

    pub fn add(&self, other: &Coord) -> Self {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn toward(&self, other: &Coord) -> Self {
        Coord {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }

    pub fn subtract(&self, other: &Coord) -> Self {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn normalize(&self) -> Self {
        Coord {
            x: if self.x != 0 {
                self.x / self.x.abs()
            } else {
                0
            },
            y: if self.y != 0 {
                self.y / self.y.abs()
            } else {
                0
            },
        }
    }

    pub fn distance(&self, other: &Coord) -> i32 {
        let diff = self.subtract(other);
        ((diff.x.pow(2) + diff.y.pow(2)) as f32).sqrt().round() as i32
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_new() -> Result<(), String> {
        assert_eq!(Coord { x: 1, y: 2 }, Coord::new(1, 2));
        Ok(())
    }

    #[test]
    fn test_add() -> Result<(), String> {
        let coord = Coord::new(1, 2);
        assert_eq!(Coord { x: 3, y: 5 }, coord.add(&Coord::new(2, 3)));
        Ok(())
    }

    #[test]
    fn test_subtract() -> Result<(), String> {
        let coord = Coord::new(8, 10);
        assert_eq!(Coord { x: 6, y: 7 }, coord.subtract(&Coord::new(2, 3)));
        Ok(())
    }

    #[test]
    fn test_normalize() -> Result<(), String> {
        assert_eq!(Coord { x: 0, y: 0 }, Coord::new(0, 0).normalize());
        assert_eq!(Coord { x: 1, y: 0 }, Coord::new(100, 0).normalize());
        assert_eq!(Coord { x: 1, y: 1 }, Coord::new(2, 3).normalize());
        assert_eq!(Coord { x: -1, y: 1 }, Coord::new(-2, 3).normalize());
        Ok(())
    }

    #[test]
    fn test_distance() -> Result<(), String> {
        assert_eq!(1, Coord::new(0, 0).distance(&Coord::new(1, 1)));
        assert_eq!(4, Coord::new(0, 0).distance(&Coord::new(2, 3)));
        Ok(())
    }
}
