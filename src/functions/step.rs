use crate::data::{Entity, Event, State};

use super::{cleanup, physics, spawn};


pub fn step(state:&mut State, is_server:bool, delta:f32, iterations:i32, events:&[Event])
{
    cleanup::step(state);
    if is_server {
        state.time += delta;
        spawn::step(state, delta, iterations, events);
    }

    let mut slice:Vec<&mut Entity> = state.entities.iter_mut().collect();
    let physics_events = events.iter().filter_map(|x| {
        if let Event::PhysicsEventFromPlayer(player_id, e) = x {
            return Some(*e);
        }

        None
    });
    physics::step(&mut slice, is_server, physics_events);
}