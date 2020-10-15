// macro_use needs to go first so the macro is visible for the other modules
#[macro_use]

mod player;
mod world;

pub use self::player::{Player};
pub use self::world::World;