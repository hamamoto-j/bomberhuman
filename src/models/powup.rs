use crate::geometry::Point;

#[derive(Debug, PartialEq, Eq)]
pub struct Powup {
    pub pos: Point,
    pub ttl: i32,
    pub types: i32,
    pub spr_idx: i32,
}

impl Powup {
    pub fn new(pos: Point, types: i32) -> Powup {
        Powup {
            pos: pos,
            ttl: 15,
            types: types,
            spr_idx: 0,
        }
    }

    pub fn update(&mut self) {
        self.ttl = self.ttl - 1;

        //spr_idxの更新
        if self.ttl < 15 {
            self.spr_idx = 0;
        }
        if self.ttl < 10 {
            self.spr_idx = 1;
        }
        if self.ttl < 5 {
            self.spr_idx = 2;
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

    pub fn ttl(&self) -> i32 {
        self.ttl
    }

    pub fn position(&self) -> Point {
        self.pos
    }
}
