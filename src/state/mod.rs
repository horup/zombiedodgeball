mod entity;
pub use entity::*;
mod actor;
pub use actor::*;
mod input;
pub use input::*;

#[derive(Clone)]
pub struct State {
    pub entities:gamestate::Entities<Entity>,
    pub input:Input
}

impl State {
    pub fn new() -> Self {
        Self {
            entities:gamestate::Entities::new(),
            input:Input::default()
        }
    }
}

