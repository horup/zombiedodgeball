use cgmath::Vector2;

#[derive(Copy, Clone, Debug)]
pub enum Event
{
    PlayerSpawn(u128),
    PlayerMove(u128, Vector2<f32>)
}
