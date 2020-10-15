#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Keys{
    pub arrow_up: bool,
    pub arrow_down: bool,
    pub arrow_right: bool,
    pub arrow_left: bool,
    pub space: bool
}

impl Keys{
    pub fn new() -> Keys{
        Keys{
            arrow_up : false,
            arrow_down : false,
            arrow_right : false,
            arrow_left : false,
            space : false
        }
    }
}