use crate::data::{Event, State};

use super::{cleanup, physics, spawn};


pub fn step(state:&mut State, is_server:bool, delta:f32, iterations:i32, events:&[Event])
{
    cleanup::step(state);
    if is_server {
        state.time += delta;
        spawn::step(state, delta, iterations, events);
    }

    physics::step(state, is_server, events);
}