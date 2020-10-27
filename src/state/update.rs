use super::{Actor, State};

impl State
{
    pub fn input_update(&mut self)
    {
        let input = &self.input;
        let v = input.dpad;
        let speed = 0.2;
        for (id, e) in self.entities.iter_mut()
        {
            if let Actor::Player(player) = e.actor
            {
                e.vel = v * speed;
            }
        }

    }
    pub fn update(&mut self)
    {
        self.input_update();
        for (id, e) in self.entities.iter_mut()
        {
            e.pos = e.pos + e.vel;
        }
    }
}