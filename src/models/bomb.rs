use crate::geometry::Point;

#[derive(Debug, PartialEq, Eq)]
pub struct Bomb{
    pub pos: Point,
    pub ttl: i32, 
    pub player_idx: i32,
}

impl Bomb{
    pub fn new(pos: Point, player_idx: i32) -> Bomb{
        Bomb {
            pos: pos,
            ttl: 180,
            player_idx: player_idx,
        }
    }

    pub fn update(&mut self){
        self.ttl = self.ttl -1;
    }

    pub fn is_alive(&self) -> bool{
        self.ttl > 0 
    }

    pub fn x(&self) -> i32{
        self.pos.x
    }

    pub fn y(&self) -> i32{
        self.pos.y
    }

    pub fn ttl(&self) -> i32{
        self.ttl
    }

    pub fn position(&self) -> Point{
        self.pos
    }
}