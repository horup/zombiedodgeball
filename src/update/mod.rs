mod spawn;
mod cleanup;
mod physics;
mod update;
use cgmath::Vector2;
pub use update::*;

#[derive(Copy, Clone, Debug)]
pub enum Event
{
    PlayerSpawn(u128),
    PlayerMove(u128, Vector2<f32>)
}
