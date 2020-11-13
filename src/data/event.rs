use cgmath::{Point2};
use gamestate::ID;

use crate::systems::physics::PhysicsEvent;


#[derive(Copy, Clone, Debug)]
pub enum Event
{
    PlayerSpawn(u128),
    PhysicsEventFromPlayer(u128, PhysicsEvent),
    ShootAt(ID, Point2<f32>),
    Tick(i32, f32)
}
