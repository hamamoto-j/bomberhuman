mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Keys{
    arrow_up: bool,
    arrow_down: bool,
    arrow_right: bool,
    arrow_left: bool
}


#[wasm_bindgen]
impl Keys{
    pub fn new() -> Keys{
        Keys{
            arrow_up : false,
            arrow_down : false,
            arrow_right : false,
            arrow_left : false,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player{
    x: i32,
    y: i32,
    speed: i32,
}

#[wasm_bindgen]
impl Player{
    pub fn new(x: i32, y: i32, speed: i32) -> Player{
        Player {
            x: x, 
            y: y, 
            speed: speed
        }
    }

    pub fn update(&mut self, keys : Keys) {
        if keys.arrow_up == true {
            self.y -= self.speed;
        }
        if keys.arrow_down {
            self.y += self.speed;
        } 
        if keys.arrow_right {
            self.x += self.speed;
        } 
        if keys.arrow_left {
            self.x -= self.speed;
        }   
    }

    pub fn x(&self) -> i32{
        self.x
    }

    pub fn y(&self) -> i32{
        self.y
    }

    pub fn speed(&self) -> i32{
        self.speed
    }
}


#[wasm_bindgen]
pub struct GameState{
    width: i32,
    height: i32,
    player: Player,
    keys: Keys,
}

#[wasm_bindgen]
impl GameState{
    pub fn new() -> GameState{
        GameState{
            width: 1024,
            height: 1024,
            player: Player::new(50,50,5),
            keys: Keys::new(),
        }
    }

    pub fn action(&mut self, s: &str, b: bool){
        match s{
            "ArrowUp" => self.keys.arrow_up = b,
            "ArrowDown" => self.keys.arrow_down = b,
            "ArrowRight" => self.keys.arrow_right = b,
            "ArrowLeft" => self.keys.arrow_left = b,
            _ => (),
        }
    }

    pub fn update(&mut self){
        self.player.update(self.keys)
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn player(&self) -> Player{
        self.player
    }

    pub fn p_x(&self) -> i32{
        self.player.x()
    }

    pub fn p_y(&self) -> i32{
        self.player.y()
    }
}