mod entity;
pub use entity::*;
mod actor;
pub use actor::*;
mod sprite;
pub use sprite::*;

#[derive(Clone)]
pub struct State {
    pub entities:gamestate::Entities<Entity>
}

impl State {
    pub fn new() -> Self {
        Self {
            entities:gamestate::Entities::new()
        }
    }
}

