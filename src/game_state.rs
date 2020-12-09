use crate::controllers::CollisionController;
use crate::geometry::Point;
use crate::models::Bomb;
use crate::models::Fire;
use crate::models::Player;
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
        //Player Vec をループ:player を　update
        for player in &mut self.world.player {
            player.pre_pos = player.pos;
            player.update(self.world.keys[player.id as usize]);
        }

        let mut players: [Player; 4] = [
            Player::new(Point::new(0, 0), 0),
            Player::new(Point::new(0, 0), 0),
            Player::new(Point::new(0, 0), 0),
            Player::new(Point::new(0, 0), 0),
        ];
        players.clone_from_slice(&self.world.player);

        for player in &mut self.world.player {
            //bomb の設置
            if player.put_bomb_state() && player.bomb_num >= 1 {
                let player_idx = utils::pos_to_idx(player.position());
                match self.world.obj.get(player_idx).unwrap() {
                    0 => {
                        let p_pos = utils::idx_to_pos(player_idx);
                        let mut p_list = Vec::new();
                        for p in &players {
                            if CollisionController::player_to_bomb(&p_pos, &p.pos) {
                                p_list.push(p.id);
                            }
                        }
                        // 0 = Nothing
                        self.world.obj.remove(player_idx);
                        self.world.obj.insert(player_idx, 4); // 4 = Bomb
                        self.world.bomb.push(Bomb::new(
                            utils::idx_to_pos(player_idx),
                            player.id,
                            p_list,
                        ));
                        player.bomb_num -= 1;
                    }
                    _ => (),
                }
            }
        }

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
                    18, //base_ttl
                    self.world.player[bomb.player_idx as usize].fire_num,
                    0,
                ));
                // --> Fire::new(position, base_ttl, spread_t, child, direction)
                self.world.player[bomb.player_idx as usize].bomb_num += 1;
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
                            1, // 1 = up
                        ));
                        fire_queue.push(Fire::new(
                            fire.position(),
                            fire.base_ttl,
                            fire.spread_t,
                            fire.child - 1,
                            2, // 2=right
                        ));
                        fire_queue.push(Fire::new(
                            fire.position(),
                            fire.base_ttl,
                            fire.spread_t,
                            fire.child - 1,
                            3, // 3=down
                        ));
                        fire_queue.push(Fire::new(
                            fire.position(),
                            fire.base_ttl,
                            fire.spread_t,
                            fire.child - 1,
                            4, //4 = left
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
                    // 1 = wall
                    fire.ttl = 0;
                }
                2 => {
                    // 2 = brock
                    fire.ttl = 0;
                    for brock in &mut self.world.brock {
                        if utils::is_eq_pos(fire.pos, brock.pos) {
                            brock.is_broken = true;
                        }
                    }
                }
                4 => {
                    // 4 = bomb
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

        //brockの更新とマップの更新
        for brock in &mut self.world.brock {
            brock.update();
            if !brock.is_alive() {
                let brock_idx = utils::pos_to_idx(brock.pos);
                self.world.obj.remove(brock_idx);
                self.world.obj.insert(brock_idx, 0); // 0 = non
            }
        }

        //寿命が0になった brock の削除
        self.world.brock.retain(|x| x.is_alive());

        //寿命が0になった bomb の削除
        self.world.bomb.retain(|x| x.is_alive());

        //寿命が0になった fire の削除
        self.world.fire.retain(|x| x.is_alive());

        //fireのcollision
        for fire in &mut self.world.fire {
            for pow in &mut self.world.pow {
                if utils::is_eq_pos(fire.pos, pow.pos) {
                    fire.ttl = 0;
                    pow.ttl = 0;
                }
            }
        }

        //寿命が0になった pow の削除
        self.world.pow.retain(|x| x.is_alive());

        //各プレイヤーの衝突判定
        for player in &mut self.world.player {
            //wall collision 判定
            for (i, obj_num) in self.world.obj.iter().enumerate() {
                let obj_pos = utils::idx_to_pos(i);

                match obj_num {
                    //     Non = 0,
                    //     Wall = 1,
                    //     Brock = 2,
                    //     Pow = 3,
                    //     Bomb = 4,
                    //     Brock + Pow = 5,
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
                    2 => {
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

            for bomb in &mut self.world.bomb {
                if CollisionController::player_to_bomb(&player.pos, &bomb.pos) {
                    if !bomb.player_list.contains(&player.id) {
                        //bombのplayer_listに含まれていないidのプレイヤーの場合＝＞衝突
                        let mut move_point =
                            CollisionController::player_to_wall_horizonal(&player.pos, &bomb.pos);
                        player.move_to(move_point);

                        move_point =
                            CollisionController::player_to_wall_vertical(&player.pos, &bomb.pos);
                        player.move_to(move_point);
                    }
                } else {
                    bomb.player_list.retain(|&x| x != player.id);
                }
            }

            for pow in &mut self.world.pow {
                if CollisionController::player_to_bomb(&player.pos, &pow.pos) {
                    pow.ttl = 0;
                    player.pow_up(pow.id);
                }
            }
        }

        self.world.timer.update();
    }

    pub fn width(&self) -> i32 {
        self.world.width
    }

    pub fn height(&self) -> i32 {
        self.world.height
    }
}
