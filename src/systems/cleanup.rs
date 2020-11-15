use crate::{event::Event, world::World};

pub fn step<F:FnMut(Event)>(state:&mut World, is_server:bool, event:&Event, push_event:&mut F)
{
    match event 
    {
        Event::DeleteEntity(id) => {
            state.entities.delete_entity(*id);
        },
        Event::Tick(_, _) => {
            for e in state.entities.iter() {
                let mut f = || {
                   // println!("{}", e.missile?.exploded);
                    if e.missile?.exploded {
                        println!("test");
                        push_event(Event::DeleteEntity(e.id));
                    }
                    Some(())
                };
                f();
            }
        },
        _ => {}
    }
}