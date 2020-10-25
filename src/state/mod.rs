mod entity;
pub use entity::*;
mod actor;
pub use actor::*;



pub type State = gamestate::State<Entity>;

