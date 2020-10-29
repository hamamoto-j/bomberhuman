use crate::geometry::Point;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Wall{
    pos: Point,
}

impl Wall{
    pub fn new(x: i32, y: i32) -> Wall{
        Wall {
            pos: Point::new(x,y),
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