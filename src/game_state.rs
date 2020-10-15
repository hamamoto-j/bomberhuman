use crate::models::World;

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
            _ => (),
        }
    }

    pub fn update(&mut self){
        self.world.player.update(self.world.keys);
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