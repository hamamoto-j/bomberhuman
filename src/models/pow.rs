use crate::geometry::Point;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pow{
    pos: Point,
    id: i32,
}

impl Pow{
    pub fn new(pos: Point, id: i32) -> Pow{
        Pow {
            pos: pos,
            id: id,
        }
    }
    
    pub fn x(&self) -> i32{
        self.pos.x
    }

    pub fn y(&self) -> i32{
        self.pos.y
    }

    pub fn position(&self) -> Point{
        self.pos
    }
}