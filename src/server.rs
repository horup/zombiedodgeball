use crate::state::State;

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
        self.current.clone()
    }
}