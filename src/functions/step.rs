use crate::data::{Entity, Event, State};

use super::{cleanup, physics, spawn};


pub fn step(state:&mut State, is_server:bool, delta:f32, iterations:i32, events:&[Event])
{
    cleanup::step(state);
    if is_server {
        state.time += delta;
        spawn::step(state, delta, iterations, events);
    }

    for e in state.entities.iter() {
        
    }
    let mut slice:Vec<&mut Entity> = state.entities.iter_mut().collect();
    physics::step(&mut slice, is_server, events);
}