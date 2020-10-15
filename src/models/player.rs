use crate::controllers::Keys;
use crate::geometry::Point;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player{
    pos: Point,
    speed: i32,
}

impl Player{
    pub fn new(x: i32, y: i32, speed: i32) -> Player{
        Player {
            pos: Point::new(x,y),
            speed: speed,
        }
    }

    pub fn update(&mut self, keys : Keys) {
        if keys.arrow_up == true {
            self.pos.y -= self.speed;
        }
        if keys.arrow_down {
            self.pos.y += self.speed;
        } 
        if keys.arrow_right {
            self.pos.x += self.speed;
        } 
        if keys.arrow_left {
            self.pos.x -= self.speed;
        }   
    }

    pub fn x(&self) -> i32{
        self.pos.x
    }

    pub fn y(&self) -> i32{
        self.pos.y
    }
}