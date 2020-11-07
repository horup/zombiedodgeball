use crate::data::State;

use super::{Event, cleanup, physics, spawn};


pub fn update(state:&mut State, is_server:bool, delta:f32, iterations:i32, events:&[Event])
{
    cleanup::cleanup(state);
    if is_server {
        state.time += delta;
        spawn::spawn(state, delta, iterations, events);
    }

    physics::physics(state, is_server, events);

    //spawn::spawn(state, delta, iterations, wish_to_join)
}