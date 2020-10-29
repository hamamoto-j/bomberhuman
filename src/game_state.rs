use crate::models::World;
use crate::models::Bomb;
use crate::controllers::CollisionController;
use crate::geometry::Point;

use crate::utils;

pub struct GameState{
    pub world: World,
}

impl GameState{
    pub fn new() -> GameState{
        GameState{
            world: World::new(),
        }
    }

    pub fn action(&mut self, s: &str, b: bool){
        match s{
            "ArrowUp" => self.world.keys.arrow_up = b,
            "ArrowDown" => self.world.keys.arrow_down = b,
            "ArrowRight" => self.world.keys.arrow_right = b,
            "ArrowLeft" => self.world.keys.arrow_left = b,
            " " => self.world.keys.space= b,
            _ => (),
        }
    }

    pub fn update(&mut self){
        self.world.player.update(self.world.keys);

        for bomb in &mut self.world.bomb{
            bomb.update();
            // bomb.ttl = bomb.ttl -1;
            if !bomb.is_alive(){
                let bomb_idx = utils::pos_to_idx(bomb.position());
                self.world.obj.remove(bomb_idx);
                self.world.obj.insert(bomb_idx, 0); // 0 = Nothing
            }
        }

        self.world.bomb.retain(|x| x.is_alive()); 

        if self.world.player.put_bomb_state(){
            let player_idx = utils::pos_to_idx(self.world.player.position());
            match self.world.obj.get(player_idx).unwrap() {
                0 =>{// 0 = Nothing
                    self.world.obj.remove(player_idx);
                    self.world.obj.insert(player_idx, 4); // 4 = Bomb
                    self.world.bomb.push(Bomb::new(utils::idx_to_pos(player_idx)));
                },
                _ => (),
            }
        }

        for (i, obj_num) in self.world.obj.iter().enumerate() {
            let obj_pos = utils::idx_to_pos(i);

            match obj_num {
                1 => {let mut move_point = CollisionController::player_to_wall_horizonal(&self.world.player.position(), &obj_pos);
                    self.world.player.move_to(move_point);
        
                    move_point = CollisionController::player_to_wall_vertical(&self.world.player.position(), &obj_pos);
                    self.world.player.move_to(move_point);
        
                    move_point = CollisionController::player_to_wall_corner(&self.world.player.position(), &obj_pos);
                    self.world.player.move_to(move_point);},
                _ => (),
            }
        }
    }

    pub fn width(&self) -> i32 {
        self.world.width
    }

    pub fn height(&self) -> i32 {
        self.world.height
    }

    pub fn p_x(&self) -> i32{
        self.world.player.x()
    }

    pub fn p_y(&self) -> i32{
        self.world.player.y()
    }
}