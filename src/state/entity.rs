use cgmath::Vector2;
use gamestate::DeltaSerializable;
use super::{Actor, Player, Sprite};

#[derive(Copy, Clone, PartialEq)]
pub struct Entity
{
    pub pos:Vector2<f32>,
    pub vel:Vector2<f32>,
    pub sprite:Option<Sprite>,
    pub actor:Option<Actor>,
    pub player:Option<Player>
}

impl Default for Entity
{
    fn default() -> Self {
        Entity {
            pos:Vector2 {x:0.0, y:0.0},
            vel:Vector2 {x:0.0, y:0.0},
            actor:None,
            sprite:None,
            player:None
        }
    }
}

impl DeltaSerializable  for Entity
{
    fn delta_serialize(&self, previous:&Self, write:&mut dyn std::io::Write) -> std::io::Result<usize> {
        todo!()
    }

    fn delta_deserialize(previous:&Self, read:&mut dyn std::io::Read) -> std::io::Result<Self> where Self : Sized {
        todo!()
    }
}