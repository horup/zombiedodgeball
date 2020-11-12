use cgmath::Vector2;

use crate::functions::physics::PhysicsEvent;

#[derive(Copy, Clone, Debug)]
pub enum Event
{
    PlayerSpawn(u128),
    PhysicsEventFromPlayer(u128, PhysicsEvent)
}
