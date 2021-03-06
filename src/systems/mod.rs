use crate::{event::Event, world::World};

pub mod util;
pub mod spawn;
pub mod cleanup;
pub mod physics;
pub mod shooter;

pub fn step(state:&mut World, is_server:bool, events:&Vec<Event>)
{
    let mut new_events = Vec::new();
    let systems = 
    [
        cleanup::step,
        physics::step,
        spawn::step,
        shooter::step
    ];

    let mut f = |e| new_events.push(e);
    for system in systems.iter() {
        for e in events {
            system(state, is_server, e, &mut f);
        }
        
    }

    if new_events.len() > 0 {
        step(state, is_server, &new_events);
    }
}