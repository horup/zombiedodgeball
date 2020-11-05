use crate::{ClientData, state::{State}};
use crate::systems;
mod update;

pub struct Server {
    pub current:State,
    pub iterations:i32
}

impl Server {
    pub fn new() -> Self
    {
        Self {
            current:State::new(),
            iterations:0
        }
    }

    pub fn update(&mut self, delta:f32, client_data:&[ClientData]) -> State
    {
        systems::cleanup::cleanup(&mut self.current);
        systems::spawn::spawn(&mut self.current, self.iterations);
        update::update_clients(&mut self.current, delta, client_data);

        
        let collisions = update::update_movement(&mut self.current, delta);
        update::update_dodge_ball(&mut self.current, delta, &collisions);
        update::update_actors(&mut self.current, delta);

        self.iterations += 1;
        self.current.clone()
    }
    
    pub fn restart(&mut self) {
        self.current.entities.clear();
    }
}