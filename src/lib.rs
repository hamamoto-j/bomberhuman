mod controllers;
mod game_state;
mod geometry;
mod models;

use crate::controllers::Keys;
use crate::models::Player;

use wasm_bindgen::prelude::*;

use std::os::raw::c_int;
mod utils;

use self::game_state::GameState;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

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
        init_panic_hook();
        let draw = Draw::new();
        let width = draw.width(512);
        let height = draw.height(512);
        GameData {
            state: GameState::new(),
        }
    }

    pub fn add_player(&mut self, player_idx: i32) {
        for i in 0..player_idx {
            let new_player = Player::new(utils::players_pos(i), i);
            self.state.world.player.push(new_player);
            self.state.world.keys.push(Keys::new());
        }
    }

    pub fn update(&mut self) {
        self.state.update();
    }

    pub fn action(&mut self, s: &str, b: bool) {
        self.state.action(s, b);
    }

    pub fn width(&self) -> i32 {
        self.state.width()
    }

    pub fn height(&self) -> i32 {
        self.state.height()
    }

    pub fn draw(&mut self) {
        let draw = Draw::new();

        draw.clear_screen();

        for player in &self.state.world.player {
            if player.is_alive {
                draw.draw_player(player.x(), player.y());
            }
        }

        for (i, obj_num) in self.state.world.obj.iter().enumerate() {
            //     Non = 0,
            //     Wall = 1,
            //     Brock = 2,
            //     Pow = 3,
            //     Bomb = 4,
            //     Brock + Pow = 5,
            let obj_point = utils::idx_to_pos(i);
            match obj_num {
                1 => draw.draw_wall(obj_point.x, obj_point.y),
                _ => (),
            }
        }

        for bomb in &self.state.world.bomb {
            draw.draw_bomb(bomb.x(), bomb.y());
        }

        for fire in &self.state.world.fire {
            draw.draw_fire(fire.x(), fire.y());
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
    pub fn draw_fire(this: &Draw, _: c_int, _: c_int);

    #[wasm_bindgen(method)]
    pub fn draw_wall(this: &Draw, _: c_int, _: c_int);
}
