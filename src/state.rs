use crate::entity::Entity;

#[derive(Clone)]
pub struct State {
    pub time:f32,
    pub entities:gamestate::Collection<Entity>
}

impl State {
    pub fn new() -> Self {
        Self {
            time:0.0,
            entities:gamestate::Collection::default()
        }
    }
}

