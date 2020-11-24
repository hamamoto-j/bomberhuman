use crate::controllers::CollisionController;
use crate::models::Bomb;
use crate::models::Fire;
use crate::models::World;

use crate::utils;

pub struct GameState {
    pub world: World,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            world: World::new(),
        }
    }

    pub fn action(&mut self, s: &str, b: bool) {
        match s {
            "ArrowUp" => self.world.keys[0].arrow_up = b,
            "ArrowDown" => self.world.keys[0].arrow_down = b,
            "ArrowRight" => self.world.keys[0].arrow_right = b,
            "ArrowLeft" => self.world.keys[0].arrow_left = b,
            " " => self.world.keys[0].space = b,
            "w" => self.world.keys[1].arrow_up = b,
            "s" => self.world.keys[1].arrow_down = b,
            "d" => self.world.keys[1].arrow_right = b,
            "a" => self.world.keys[1].arrow_left = b,
            "q" => self.world.keys[1].space = b,
            "t" => self.world.keys[2].arrow_up = b,
            "g" => self.world.keys[2].arrow_down = b,
            "h" => self.world.keys[2].arrow_right = b,
            "f" => self.world.keys[2].arrow_left = b,
            "r" => self.world.keys[2].space = b,
            "i" => self.world.keys[3].arrow_up = b,
            "k" => self.world.keys[3].arrow_down = b,
            "l" => self.world.keys[3].arrow_right = b,
            "j" => self.world.keys[3].arrow_left = b,
            "u" => self.world.keys[3].space = b,
            _ => (),
        }
    }

    pub fn update(&mut self) {
        //bomb の更新
        for bomb in &mut self.world.bomb {
            bomb.update();
            // bomb.ttl = bomb.ttl -1;
            if !bomb.is_alive() {
                let bomb_idx = utils::pos_to_idx(bomb.position());
                self.world.obj.remove(bomb_idx);
                self.world.obj.insert(bomb_idx, 0); // 0 = non
                self.world.fire.push(Fire::new(
                    bomb.position(),
                    20,
                    18,
                    self.world.player[bomb.player_idx as usize].fire_num,
                    0,
                ));
                // --> Fire::new(position, base_ttl, spread_t, child, direction)
                self.world.player[bomb.player_idx as usize].bomb_num += 1;
            }
        }

        //Player Vec をループ
        for player in &mut self.world.player {
            player.update(self.world.keys[player.id as usize]);

            //bomb の設置
            if player.put_bomb_state() && player.bomb_num >= 1 {
                let player_idx = utils::pos_to_idx(player.position());
                match self.world.obj.get(player_idx).unwrap() {
                    0 => {
                        // 0 = Nothing
                        self.world.obj.remove(player_idx);
                        self.world.obj.insert(player_idx, 4); // 4 = Bomb
                        self.world
                            .bomb
                            .push(Bomb::new(utils::idx_to_pos(player_idx), player.id));
                        player.bomb_num -= 1;
                    }
                    _ => (),
                }
            }
        }

        let mut fire_queue = Vec::new();

        //fireの更新
        for fire in &mut self.world.fire {
            fire.update();
            if fire.ttl == fire.spread_t && fire.child > 0 {
                match fire.direction {
                    0 => {
                        fire_queue.push(Fire::new(
                            fire.position(),
                            fire.base_ttl,
                            fire.spread_t,
                            fire.child - 1,
                            1,
                        ));
                        fire_queue.push(Fire::new(
                            fire.position(),
                            fire.base_ttl,
                            fire.spread_t,
                            fire.child - 1,
                            2,
                        ));
                        fire_queue.push(Fire::new(
                            fire.position(),
                            fire.base_ttl,
                            fire.spread_t,
                            fire.child - 1,
                            3,
                        ));
                        fire_queue.push(Fire::new(
                            fire.position(),
                            fire.base_ttl,
                            fire.spread_t,
                            fire.child - 1,
                            4,
                        ));
                    }
                    _ => {
                        fire_queue.push(Fire::new(
                            fire.position(),
                            fire.base_ttl - 1,
                            fire.spread_t - 2,
                            fire.child - 1,
                            fire.direction,
                        ));
                    }
                }
            }
        }
        //fire_queueの消化
        for fire in fire_queue {
            self.world.fire.push(fire);
        }

        //fireのcollision
        for fire in &mut self.world.fire {
            match self.world.obj[utils::pos_to_idx(fire.pos)] {
                1 => {
                    fire.ttl = 0;
                }
                4 => {
                    fire.ttl = 0;
                    for bomb in &mut self.world.bomb {
                        if utils::is_eq_pos(fire.pos, bomb.pos) {
                            bomb.ttl = 1;
                        }
                    }
                }
                _ => (),
            }
        }

        //寿命が0になった bomb の削除
        self.world.bomb.retain(|x| x.is_alive());

        //寿命が0になった fire の削除
        self.world.fire.retain(|x| x.is_alive());

        //各プレイヤーの衝突判定
        for player in &mut self.world.player {
            //collision 判定
            for (i, obj_num) in self.world.obj.iter().enumerate() {
                let obj_pos = utils::idx_to_pos(i);

                match obj_num {
                    1 => {
                        let mut move_point = CollisionController::player_to_wall_horizonal(
                            &player.position(),
                            &obj_pos,
                        );
                        player.move_to(move_point);

                        move_point = CollisionController::player_to_wall_vertical(
                            &player.position(),
                            &obj_pos,
                        );
                        player.move_to(move_point);

                        move_point = CollisionController::player_to_wall_corner(
                            &player.position(),
                            &obj_pos,
                        );
                        player.move_to(move_point);
                    }
                    _ => (),
                }
            }

            for fire in &self.world.fire {
                if CollisionController::player_to_fire(&player.pos, &fire.pos) {
                    player.is_alive = false;
                }
            }
        }
    }

    pub fn width(&self) -> i32 {
        self.world.width
    }

    pub fn height(&self) -> i32 {
        self.world.height
    }
}
