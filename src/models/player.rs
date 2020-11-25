use std::cmp::min;

use crate::controllers::Keys;
use crate::geometry::Point;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player {
    pub pos: Point,
    pub speed: i32,
    pub bomb_num: i32,
    pub fire_num: i32,
    put_bomb: bool,
    pub id: i32,
    pub is_alive: bool,
}

impl Player {
    pub fn new(pos: Point, id: i32) -> Player {
        Player {
            pos: pos,
            speed: 1,
            bomb_num: 1,
            fire_num: 1,
            put_bomb: false,
            id: id,
            is_alive: true,
        }
    }

    pub fn update(&mut self, keys: Keys) {
        if self.is_alive {
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
        } else {
            self.pos.y = 0;
            self.pos.x = 0;
            self.put_bomb = false;
        }
    }

    pub fn pow_up(&mut self, pow_id: i32) {
        match pow_id {
            1 => self.speed = min(self.speed + 1, 9),
            2 => self.bomb_num = min(self.bomb_num + 1, 9),
            3 => self.fire_num = min(self.fire_num + 1, 9),
            _ => {}
        }
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

    pub fn put_bomb_state(&self) -> bool {
        self.put_bomb
    }

    pub fn move_to(&mut self, move_point: Point) {
        self.pos.x = move_point.x;
        self.pos.y = move_point.y;
    }
}
