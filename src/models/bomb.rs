use crate::geometry::Point;

#[derive(Debug, PartialEq, Eq)]
pub struct Bomb {
    pub pos: Point,
    pub ttl: i32,
    pub player_idx: i32,
    pub player_list: Vec<i32>,
    pub spr_idx: i32,
}

impl Bomb {
    pub fn new(pos: Point, player_idx: i32, player_list: Vec<i32>) -> Bomb {
        Bomb {
            pos: pos,
            ttl: 180,
            player_idx: player_idx,
            player_list: player_list,
            spr_idx: 0,
        }
    }

    pub fn update(&mut self) {
        self.ttl = self.ttl - 1;

        //spr_idxの更新
        if self.ttl < 180 {
            self.spr_idx = 0;
        }
        if self.ttl < 150 {
            self.spr_idx = 1;
        }
        if self.ttl < 120 {
            self.spr_idx = 2;
        }
        if self.ttl < 90 {
            self.spr_idx = 3;
        }
        if self.ttl < 60 {
            self.spr_idx = 4;
        }
        if self.ttl < 30 {
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
