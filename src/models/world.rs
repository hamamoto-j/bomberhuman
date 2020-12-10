use crate::models::Player;
// use crate::models::Wall;
use crate::controllers::Keys;
use crate::models::Bomb;
use crate::models::Brock;
use crate::models::Fire;
use crate::models::Particle;
use crate::models::Pow;
use crate::models::Powup;
use crate::models::Timer;

pub struct World {
    pub player: Vec<Player>,
    pub bomb: Vec<Bomb>,
    pub fire: Vec<Fire>,
    pub brock: Vec<Brock>,
    pub pow: Vec<Pow>,
    pub obj: Vec<u32>,
    pub keys: Vec<Keys>,
    pub timer: Timer,
    pub width: i32,
    pub height: i32,
    pub powup: Vec<Powup>,
    pub particle: Vec<Particle>,
}

impl World {
    /// Returns a new world of the given size
    pub fn new() -> World {
        //     Player_space = 6,
        //     Non = 0,
        //     Wall = 1,
        //     Brock = 2,
        //     Pow = 3,
        //     Bomb = 4,
        //     Brock + Pow = 5,
        World {
            player: Vec::new(),
            bomb: Vec::new(),
            fire: Vec::new(),
            brock: Vec::new(),
            pow: Vec::new(),
            obj: vec![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 6, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6,
                6, 1, 1, 6, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 6, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 6, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 6, 1, 1, 6, 6,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 6, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1,
                /*
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 6, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 6, 1,
                1, 6, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 6, 1,
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                1, 6, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 6, 1,
                1, 6, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 6, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                */
            ],
            keys: Vec::new(),
            timer: Timer::new(),
            width: 2000,
            height: 2000,
            powup: Vec::new(),
            particle: Vec::new(),
        }
    }
}
