use crate::{event::Event, state::State};

pub fn step<F:FnMut(Event)>(state:&mut State, is_server:bool, event:&Event, push_event:&mut F)
{
    if let Event::Tick(_, _) = event {
        for e in state.entities.clone().iter() {
            if e.delete == true {
                state.entities.delete_entity(e.id);
            }
        }
    }
}