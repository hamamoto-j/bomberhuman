mod controllers;
mod geometry;
mod game_state;
mod models;

use wasm_bindgen::prelude::*;

use std::os::raw::{c_int};
mod utils;

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
        let draw = Draw::new();
        let width = draw.width(512);
        let height = draw.height(512);
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

    pub fn draw(&mut self){
        let draw = Draw::new();

        draw.clear_screen();

        draw.draw_player(self.state.p_x(), self.state.p_y());

        for (i, obj_num) in self.state.world.obj.iter().enumerate() {
            //     Non = 0,
            //     Wall = 1,
            //     SoftBlock = 2,
            //     Pow = 3,
            //     Bomb = 4,
            //     Fire = 5, 
            //     Soft_Pow = 6,
            let obj_point = utils::idx_to_pos(i);
            match obj_num {
                1 => draw.draw_wall(obj_point.x, obj_point.y),
                4 => draw.draw_bomb(obj_point.x, obj_point.y),
                _ => (),
            }
        }
    }
}

#[wasm_bindgen(module = "/src/javascript/draw.js")]
extern "C" {
    type Draw;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Draw;

    #[wasm_bindgen(method)]
    pub fn width(this: &Draw, x: u32) -> u32;

    #[wasm_bindgen(method)]
    pub fn height(this: &Draw, y: u32) -> u32;

    #[wasm_bindgen(method)]
    pub fn clear_screen(this: &Draw);

    #[wasm_bindgen(method)]
    pub fn draw_player(this: &Draw, _: c_int, _: c_int);

    #[wasm_bindgen(method)]
    pub fn draw_bomb(this: &Draw, _: c_int, _: c_int);

    #[wasm_bindgen(method)]
    pub fn draw_wall(this: &Draw, _: c_int, _: c_int);
}