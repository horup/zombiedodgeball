use cgmath::{Point2, Vector2};
use gamestate::{DeltaSerializable, ID};

use crate::{components::{Actor, Dodgeball, Player, Sprite}};

#[derive(Copy, Clone, PartialEq)]
pub struct Entity
{
    pub id:ID,
    pub delete:bool,
    pub collidable:bool,
    pub pos:Point2<f32>,
    pub vel:Vector2<f32>,
    pub sprite:Option<Sprite>,
    pub actor:Option<Actor>,
    pub player:Option<Player>,
    pub dodgeball:Option<Dodgeball>
}

impl gamestate::Entity for Entity 
{
    fn new(id:ID) -> Self {
        Self {
            id:id,
            delete:false,
            collidable:true,
            pos:Point2 {x:0.0, y:0.0},
            vel:Vector2 {x:0.0, y:0.0},
            actor:None,
            sprite:None,
            player:None,
            dodgeball:None
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

#[derive(Clone)]
pub struct World {
    pub time:f32,
    pub entities:gamestate::Collection<Entity>
}

impl World {
    pub fn new() -> Self {
        Self {
            time:0.0,
            entities:gamestate::Collection::default()
        }
    }
}

