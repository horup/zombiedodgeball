use crate::state::State;
mod spawn;
mod cleanup;
mod physics;

pub enum Event
{
    PlayerSpawn(u128),
    PlayerMove(u128, f32, f32)
}

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