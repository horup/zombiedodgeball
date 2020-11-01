use crate::state::State;

pub fn movement(state:&mut State, delta:f32)
{
    for (_,e) in state.entities.iter_mut() {
        e.pos += e.vel * delta;
    }
}