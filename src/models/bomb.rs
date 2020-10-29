use crate::geometry::Point;

#[derive(Debug, PartialEq, Eq)]
pub struct Bomb{
    pos: Point,
    ttl: i32, 
}

impl Bomb{
    pub fn new(pos: Point) -> Bomb{
        Bomb {
            pos: pos,
            ttl: 10,
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