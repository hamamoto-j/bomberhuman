use crate::geometry::Point;

#[derive(Debug, PartialEq, Eq)]
pub struct Particle {
    pub pos: Point,
    pub ttl: i32,
    pub types: i32,
    pub spr_idx: i32,
}

impl Particle {
    pub fn new(pos: Point) -> Particle {
        Particle {
            pos: pos,
            ttl: 24,
            types: 0,
            spr_idx: 0,
        }
    }

    pub fn update(&mut self) {
        self.ttl = self.ttl - 1;

        //spr_idxの更新
        if self.ttl < 24 {
            self.spr_idx = 0;
        }
        if self.ttl < 20 {
            self.spr_idx = 1;
        }
        if self.ttl < 16 {
            self.spr_idx = 2;
        }
        if self.ttl < 12 {
            self.spr_idx = 3;
        }
        if self.ttl < 8 {
            self.spr_idx = 4;
        }
        if self.ttl < 4 {
            self.spr_idx = 5;
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
