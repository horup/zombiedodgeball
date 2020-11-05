use crate::state::State;


pub fn cleanup(state:&mut State)
{
    for (id, e) in state.entities.clone().iter() {
        if e.delete == true {
            state.entities.delete_entity(id);
        }
    }
}