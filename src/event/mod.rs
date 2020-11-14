use cgmath::{Point2, Vector2};
use gamestate::ID;

#[derive(Copy, Clone, Debug)]
pub enum Event
{
    PlayerSpawn(u128),
    ForceMovement(ID, Vector2<f32>),
    ShootAt(ID, Point2<f32>),
    Tick(i32, f32),
    Collision(ID, ID)
}

