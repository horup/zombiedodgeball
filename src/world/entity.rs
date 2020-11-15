use cgmath::{Point2, Vector2};
use gamestate::{DeltaSerializable, ID};

use crate::components::{Collision, Missile, Player, Shooter, Sprite};


#[derive(Copy, Clone, PartialEq)]
pub struct Entity
{
    pub id:ID,
    pub pos:Point2<f32>,
    pub vel:Vector2<f32>,
    pub sprite:Option<Sprite>,
    pub player:Option<Player>,
    pub missile:Option<Missile>,
    pub shooter:Option<Shooter>,
    pub collision:Option<Collision>
}

impl gamestate::Entity for Entity 
{
    fn new(id:ID) -> Self {
        Self {
            id:id,
            pos:Point2 {x:0.0, y:0.0},
            vel:Vector2 {x:0.0, y:0.0},
            sprite:None,
            player:None,
            missile:None,
            shooter:None,
            collision:None
        }
    }

    fn id(&self) -> ID {
        self.id
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
