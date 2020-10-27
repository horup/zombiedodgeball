use cgmath::Vector2;
use crate::state::{Actor, Player, State};

pub struct Server {
    pub current:State
}

impl Server {
    pub fn new() -> Self
    {
        Self {
            current:State::new()
        }
    }

    pub fn update(&mut self) -> State
    {
        if self.current.entities.len() == 0
        {
            self.restart();
        }

        self.current.clone()
    }
    
    pub fn restart(&mut self) {
        self.current.entities.clear();
        let (id, e) = self.current.entities.new_entity_replicated().expect("could not create entity");
        e.pos = Vector2::new(10.0, 10.0);
        e.actor = Actor::Player(Player {

        });
    }
}