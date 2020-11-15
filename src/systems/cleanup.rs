use crate::{event::Event, world::World};

pub fn step<F:FnMut(Event)>(state:&mut World, is_server:bool, event:&Event, push_event:&mut F)
{
    match event 
    {
        Event::DeleteEntity(id) => {
            state.entities.delete_entity(*id);
        },
        _ => {}
    }
 /*   if let Event::Tick(_, _) = event {
        for e in state.entities.clone().iter() {
            if e.delete == true {
                state.entities.delete_entity(e.id);
            }
        }
    }*/
}