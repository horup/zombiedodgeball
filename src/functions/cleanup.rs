use crate::data::State;

pub fn step(state:&mut State)
{
    for e in state.entities.clone().iter() {
        if e.delete == true {
            state.entities.delete_entity(e.id);
        }
    }
}