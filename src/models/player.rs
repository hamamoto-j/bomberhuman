use crate::controllers::Keys;
use crate::geometry::Point;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player{
    pos: Point,
    speed: i32,
    put_bomb: bool,
}

impl Player{
    pub fn new(x: i32, y: i32, speed: i32) -> Player{
        Player {
            pos: Point::new(x,y),
            speed: speed,
            put_bomb: false,
        }
    }

    pub fn update(&mut self, keys : Keys){
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
        self.put_bomb = keys.space;
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

    pub fn put_bomb_state(&self) -> bool{
        self.put_bomb
    }

    pub fn move_to(&mut self, move_point: Point){
        self.pos.x = move_point.x;
        self.pos.y = move_point.y;
    }
}