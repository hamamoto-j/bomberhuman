use crate::geometry::Point;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Brock {
    pub pos: Point,
    ttl: i32,
    pub is_broken: bool,
    pub spr_idx: i32,
}

impl Brock {
    pub fn new(pos: Point) -> Brock {
        Brock {
            pos: pos,
            ttl: 25,
            is_broken: false,
            spr_idx: 0,
        }
    }

    pub fn update(&mut self) {
        if self.is_broken {
            self.ttl = self.ttl - 1;
        }

        //spr_idxの更新
        if self.ttl < 24 {
            self.spr_idx = 0;
        }
        if self.ttl < 20 {
            self.spr_idx = 1;
        }
        if self.ttl < 17 {
            self.spr_idx = 2;
        }
        if self.ttl < 14 {
            self.spr_idx = 3;
        }
        if self.ttl < 11 {
            self.spr_idx = 4;
        }
        if self.ttl < 8 {
            self.spr_idx = 5;
        }
        if self.ttl < 5 {
            self.spr_idx = 6;
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
