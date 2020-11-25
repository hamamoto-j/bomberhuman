use crate::geometry::Point;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Brock {
    pub pos: Point,
    ttl: i32,
    pub is_broken: bool,
}

impl Brock {
    pub fn new(pos: Point) -> Brock {
        Brock {
            pos: pos,
            ttl: 25,
            is_broken: false,
        }
    }

    pub fn update(&mut self) {
        if self.is_broken {
            self.ttl = self.ttl - 1;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.ttl > 0
    }
    pub fn x(&self) -> i32 {
        self.pos.x
    }

    pub fn y(&self) -> i32 {
        self.pos.y
    }

    pub fn position(&self) -> Point {
        self.pos
    }
}
