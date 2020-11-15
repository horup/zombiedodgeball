use cgmath::{Point2, Vector2};
use cgmath::prelude::*;
use gamestate::{DeltaSerializable, ID};

use crate::{components::{Collision, Missile, Player, Shooter, Sprite}};

use super::Entity;

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

    pub fn spawn_missile(&mut self, pos:Point2<f32>, at:Point2<f32>) -> Option<&mut Entity> {
        let e = self.entities.new_entity_replicated()?;
        e.pos = pos;
        let v = at - pos;
        let v = v.normalize() * 10.0;
        e.vel = v;
        e.missile = Some(Missile {
            ..Missile::default()
        });
        e.sprite = Some(Sprite {
            x:2.0,
            ..Sprite::default()
        });
        e.collision = Some(Collision {
            ..Collision::default()
        });
        
        Some(e)
    }

    pub fn spawn_player(&mut self, pos:Point2<f32>, player_id:u128) -> Option<&mut Entity> {
        let e = self.entities.new_entity_replicated()?;
        e.pos = pos;
        e.sprite = Some(Sprite {
            x:0.0,
            ..Sprite::default()
        });
        e.player = Some(Player {
            client_id:player_id
        });
        e.shooter = Some(Shooter {
            ..Shooter::default()
        });
        e.collision = Some(Collision {
            ..Collision::default()
        });

        Some(e)
    }

    pub fn spawn_zombie(&mut self, pos:Point2<f32>) -> Option<&mut Entity> {
        let e = self.entities.new_entity_replicated()?;
        e.pos = pos;
        e.sprite = Some(Sprite {
            x:1.0,
            ..Sprite::default()
        });
        e.collision = Some(Collision {
            ..Collision::default()
        });

        Some(e)
    }
}

