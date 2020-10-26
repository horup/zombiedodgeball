use super::State;

impl State
{
    pub fn update(&mut self)
    {
        for (id, e) in self.entities.iter_mut()
        {
            e.pos.x += 0.1;
        }
    }
}