mod controllers;
mod game_state;
mod geometry;
mod models;

use crate::controllers::Keys;
use crate::models::Brock;
use crate::models::Player;
use crate::models::Pow;

use wasm_bindgen::prelude::*;

use rand::Rng;
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
        let width = draw.width(672);
        let height = draw.height(448);
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

    pub fn add_brock(&mut self) {
        let obj_len = self.state.world.obj.len();
        let mut rng = rand::thread_rng();
        let mut obj_type: u32;
        let mut brock_num: i32 = 0;
        let mut pow_brock_num: i32 = 0;

        //Brockを規定数設置
        while brock_num < 90 {
            for i in 0..obj_len {
                match &self.state.world.obj[i] {
                    //     Player_space = 6,
                    //     Non = 0,
                    //     Wall = 1,
                    //     Brock = 2,
                    //     Pow = 3,
                    //     Bomb = 4,
                    //     Brock + Pow = 5,
                    0 => {
                        let random_num: i32 = rng.gen();
                        match random_num % 2 {
                            0 => {
                                self.state.world.obj[i] = 2; // 2 = Brock
                                brock_num += 1;
                            }
                            _ => {}
                        }
                    }
                    _ => (),
                }
            }
        }

        // Pow 付きの Brock を規定数配置
        while pow_brock_num < 30 {
            for i in 0..obj_len {
                match &self.state.world.obj[i] {
                    //     Player_space = 6,
                    //     Non = 0,
                    //     Wall = 1,
                    //     Brock = 2,
                    //     Pow = 3,
                    //     Bomb = 4,
                    //     Brock + Pow = 5,
                    2 => {
                        let random_num: i32 = rng.gen();
                        match random_num % 2 {
                            0 => {
                                self.state.world.obj[i] = 5; // 2 = Brock
                                pow_brock_num += 1;
                            }
                            _ => {}
                        }
                    }
                    _ => (),
                }
            }
        }

        //各 Brock と Powをベクタに追加
        for i in 0..obj_len {
            let obj_pos = utils::idx_to_pos(i);
            match &self.state.world.obj[i] {
                //     Player_space = 6,
                //     Non = 0,
                //     Wall = 1,
                //     Brock = 2,
                //     Pow = 3,
                //     Bomb = 4,
                //     Brock + Pow = 5,
                2 => {
                    self.state.world.brock.push(Brock::new(obj_pos));
                }
                5 => {
                    self.state.world.obj[i] = 2;
                    self.state.world.brock.push(Brock::new(obj_pos));
                    let mut pow_type: i32 = 0;
                    let random_num: i32 = rng.gen_range(0, 3);
                    match random_num {
                        0 => pow_type = 0,
                        1 => pow_type = 1,
                        2 => pow_type = 2,
                        _ => pow_type = 0,
                    }
                    self.state.world.pow.push(Pow::new(obj_pos, pow_type));
                }
                6 => {
                    self.state.world.obj[i] = 0; // 0 = Non
                }
                _ => (),
            }
        }
    }

    pub fn update(&mut self) {
        if 0 < self.state.world.timer.left() && 1 <= self.state.live_player_num() {
            self.state.update();
        } else {
        }
        if self.state.live_player_num() == 1 {
            let id: usize = self.state.live_player_id() as usize;
            self.state.world.player[id].be_immune();
            self.state.world.timer.is_work = false;
        }
    }

    pub fn action(&mut self, s: &str, b: bool) {
        self.state.action(s, b);
    }

    pub fn gp_action(&mut self, gp_id: usize, s: &str, b: bool) {
        self.state.gp_action(gp_id, s, b);
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

        for player in &mut self.state.world.player {
            if utils::is_eq_pos(player.pos, player.pre_pos) {
                player.is_animation = false;
            } else {
                player.is_animation = true;
            }

            if player.is_animation {
                player.next_spr += player.speed;
                if player.next_spr > 10 {
                    player.spr_idx = (player.spr_idx + 1) % 6;
                    player.next_spr = 0;
                }
            } else {
                player.spr_idx = 0;
                player.next_spr = 0;
            }
            if player.is_alive {
                draw.draw_player(
                    player.x(),
                    player.y(),
                    player.spr_idx,
                    player.spr_rev,
                    player.id,
                );
            }
        }

        for pow in &self.state.world.pow {
            draw.draw_pow(pow.x(), pow.y(), pow.id);
        }

        for bomb in &self.state.world.bomb {
            draw.draw_bomb(bomb.x(), bomb.y(), bomb.spr_idx);
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

        for brock in &self.state.world.brock {
            draw.draw_brock(brock.x(), brock.y(), brock.spr_idx);
        }

        for fire in &self.state.world.fire {
            draw.draw_fire(fire.x(), fire.y(), fire.spr_idx);
        }

        for powup in &self.state.world.powup {
            draw.draw_powup(powup.x(), powup.y(), powup.spr_idx, powup.types);
        }

        for particle in &self.state.world.particle {
            draw.draw_particle(particle.x(), particle.y(), particle.spr_idx);
        }

        draw.draw_timer(self.state.world.timer.left());

        if self.state.world.timer.left() <= 0 {
            draw.draw_timeup();
        }
        if self.state.live_player_num() == 1 {
            draw.draw_winner(self.state.live_player_id());
        }
        if self.state.live_player_num() == 0 {
            draw.draw_draw();
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
    pub fn draw_player(this: &Draw, _: c_int, _: c_int, _: c_int, _: c_int, _: c_int);

    #[wasm_bindgen(method)]
    pub fn draw_bomb(this: &Draw, _: c_int, _: c_int, _: c_int);

    #[wasm_bindgen(method)]
    pub fn draw_fire(this: &Draw, _: c_int, _: c_int, _: c_int);

    #[wasm_bindgen(method)]
    pub fn draw_wall(this: &Draw, _: c_int, _: c_int);

    #[wasm_bindgen(method)]
    pub fn draw_brock(this: &Draw, _: c_int, _: c_int, _: c_int);

    #[wasm_bindgen(method)]
    pub fn draw_pow(this: &Draw, _: c_int, _: c_int, _: c_int);

    #[wasm_bindgen(method)]
    pub fn draw_powup(this: &Draw, _: c_int, _: c_int, _: c_int, _: c_int);

    #[wasm_bindgen(method)]
    pub fn draw_particle(this: &Draw, _: c_int, _: c_int, _: c_int);

    #[wasm_bindgen(method)]
    pub fn draw_timer(this: &Draw, _: c_int);

    #[wasm_bindgen(method)]
    pub fn draw_timeup(this: &Draw);

    #[wasm_bindgen(method)]
    pub fn draw_winner(this: &Draw, _: c_int);

    #[wasm_bindgen(method)]
    pub fn draw_draw(this: &Draw);
}
