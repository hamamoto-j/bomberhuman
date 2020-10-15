mod utils;
mod controllers;
mod geometry;
mod game_state;
mod models;

use wasm_bindgen::prelude::*;

use self::game_state::GameState;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct GameData {
    state: GameState,
}

#[wasm_bindgen]
impl GameData {
    pub fn new() -> GameData {
        GameData {
            state: GameState::new(),
        }
    }

    pub fn update(&mut self) {
        self.state.update();
    }

    pub fn action(&mut self, s: &str, b: bool){
        self.state.action(s,b);
    }

    pub fn width(&self) -> i32 {
        self.state.width()
    }

    pub fn height(&self) -> i32 {
        self.state.height()
    }

    pub fn p_x(&mut self) -> i32 {
        self.state.p_x()
    }

    pub fn p_y(&mut self) -> i32 {
        self.state.p_y()
    }
}