use crate::data::{Event, State};

pub mod util;
pub mod spawn;
pub mod cleanup;
pub mod physics;
pub mod shooter;

pub type System = fn(&mut State, bool, &Vec<Event>) -> Vec<Event>;

pub fn step(state:&mut State, is_server:bool, events:&Vec<Event>)
{
    let mut new_events = Vec::new();
    let systems = 
    [
        cleanup::step,
        physics::step,
        spawn::step
    ];

    for system in systems.iter() {
        for new_event in system(state, is_server, events) {
            new_events.push(new_event);
        }
    }

    if new_events.len() > 0 {
        step(state, is_server, &new_events);
    }
}