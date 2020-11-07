mod entity;
pub use entity::*;
mod actor;
pub use actor::*;
mod sprite;
pub use sprite::*;
mod player;
pub use player::*;
mod dodgeball;
pub use dodgeball::*;
mod event;
pub use event::*;

#[derive(Clone)]
pub struct State {
    pub time:f32,
    pub entities:gamestate::Entities<Entity>
}

impl State {
    pub fn new() -> Self {
        Self {
            time:0.0,
            entities:gamestate::Entities::new()
        }
    }
}

