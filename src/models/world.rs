use crate::models::Player;
use crate::controllers::Keys;

pub struct World {
    pub player: Player,
    pub keys: Keys,
    pub width: i32,
    pub height: i32,
}

impl World {
    /// Returns a new world of the given size
    pub fn new() -> World {
        World {
            player: Player::new(100, 100, 5),
            keys: Keys::new(),
            width: 1024,
            height: 1024,
        }
    }
}