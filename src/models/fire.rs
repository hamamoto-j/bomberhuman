use crate::geometry::Point;

#[derive(Debug, PartialEq, Eq)]
pub struct Fire{
    pub pos: Point,
    pub ttl: i32, 
    pub base_ttl: i32,
    pub spread_t: i32,
    pub child: i32,
    pub direction: i32
}

impl Fire{
    pub fn new(pos: Point, base_ttl: i32, spread_t: i32, child: i32, direction: i32) -> Fire{
        let mut fire_pos = pos;

        match direction {
            0 => fire_pos = pos,
            1 => fire_pos = Point::new(pos.x, pos.y - 32),
            2 => fire_pos = Point::new(pos.x + 32, pos.y),
            3 => fire_pos = Point::new(pos.x, pos.y + 32),
            4 => fire_pos = Point::new(pos.x - 32, pos.y),
            _ => ()
        }

        Fire {
            pos: fire_pos,
            ttl: base_ttl,
            base_ttl: base_ttl,
            spread_t: spread_t,
            child: child,
            direction: direction
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