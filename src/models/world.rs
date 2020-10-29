use crate::models::Player;
// use crate::models::Wall;
use crate::models::Bomb;
use crate::controllers::Keys;

pub struct World {
    pub player: Player,
    pub bomb: Vec<Bomb>,
    pub obj: Vec<u32>,
    pub keys: Keys,
    pub width: i32,
    pub height: i32,
}

impl World {
    /// Returns a new world of the given size
    pub fn new() -> World {
        World {
            player: Player::new(64, 64, 3),
            bomb: Vec::new(),
            obj: vec![1,1,1,1,1,1,1,1,1,1,1,1,1,
                      1,0,0,0,0,0,0,0,0,0,0,0,1,
                      1,0,1,0,1,0,1,0,1,0,1,0,1,
                      1,0,0,0,0,0,0,0,0,0,0,0,1,
                      1,0,1,0,1,0,1,0,1,0,1,0,1,
                      1,0,0,0,0,0,0,0,0,0,0,0,1,
                      1,0,1,0,1,0,1,0,1,0,1,0,1,
                      1,0,0,0,0,0,0,0,0,0,0,0,1,
                      1,0,1,0,1,0,1,0,1,0,1,0,1,
                      1,0,0,0,0,0,0,0,0,0,0,0,1,
                      1,1,1,1,1,1,1,1,1,1,1,1,1],
            keys: Keys::new(),
            width: 1024,
            height: 1024,
        }
    }
}