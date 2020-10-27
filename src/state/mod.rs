mod entity;
use cgmath::Vector2;
pub use entity::*;
mod actor;
pub use actor::*;
mod input;
pub use input::*;
mod update;



#[derive(Clone)]
pub struct State {
    pub entities:gamestate::Entities<Entity>,
    pub input:Input
}

impl State {
    pub fn new() -> Self {
        let mut s = Self {
            entities:gamestate::Entities::new(),
            input:Input::default()
        };

        s.restart();
        s
    }

    pub fn restart(&mut self) {
        self.entities.clear();
        let (id, e) = self.entities.new_entity_replicated().expect("could not create entity");
        e.pos = Vector2::new(10.0, 10.0);
        e.actor = Actor::Player(Player {

        });
    }
}

