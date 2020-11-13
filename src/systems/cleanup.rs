use crate::data::{Event, State};

pub fn step(state:&mut State, is_server:bool, events:&Vec<Event>) -> Vec<Event>
{
    for e in events {
        if let Event::Tick(_, _) = e {
            for e in state.entities.clone().iter() {
                if e.delete == true {
                    state.entities.delete_entity(e.id);
                }
            }
        }
    }

    Vec::new()
}