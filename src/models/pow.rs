use crate::geometry::Point;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pow {
    pub pos: Point,
    pub id: i32,
    pub ttl: i32,
}

impl Pow {
    pub fn new(pos: Point, id: i32) -> Pow {
        Pow {
            pos: pos,
            id: id,
            ttl: 1,
        }
    }

    pub fn x(&self) -> i32 {
        self.pos.x
    }

    pub fn is_alive(&self) -> bool {
        self.ttl > 0
    }

    pub fn y(&self) -> i32 {
        self.pos.y
    }

    pub fn position(&self) -> Point {
        self.pos
    }
}
