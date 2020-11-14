use crate::data::{Event, State};

pub fn step(state:&mut State, is_server:bool, event:&Event) -> Vec<Event>
{
    if let Event::Tick(_, _) = event {
        for e in state.entities.clone().iter() {
            if e.delete == true {
                state.entities.delete_entity(e.id);
            }
        }
    }

    Vec::new()
}