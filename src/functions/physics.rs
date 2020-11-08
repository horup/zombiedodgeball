use cgmath::{Point2, Vector2};
use collision::{Aabb2, prelude::*};
use gamestate::{Entities, EntityID};

use crate::data::{Entity, State};
use crate::data::Event;
use super::{util::find_player_entity_mut};

pub struct Body {
    pub id:EntityID,
    pub pos:Point2<f32>,
    pub vel:Vector2<f32>
}

impl Body {
    pub fn aabb2(&self) -> Aabb2<f32> {
        let r = 0.5;
        Aabb2::new(Point2::new(self.pos.x - r, self.pos.y - r), Point2::new(self.pos.x + r, self.pos.y + r))
    }
}

impl From<&(EntityID, Entity)> for Body {
    fn from(e: &(EntityID, Entity)) -> Self {
        Self {
            id:e.0,
            pos:e.1.pos,
            vel:e.1.vel
        }
    }
}

pub fn collision_check(entity:&(EntityID, &Entity), other_entity:&(EntityID, &Entity)) -> bool
{
    let e1 = entity.1;
    let e2 = other_entity.1;
    e1.aabb2().intersects(&e2.aabb2())
}

pub fn sub_step(entity:&(EntityID, &mut Entity), all_entities:&Entities<Entity>)
{
    
}

pub fn step(state:&mut State, is_server:bool, events:&[Event])
{
    let max_step = 0.1;
   // let all_entities = state.entities;
    // update players
    for e in events {
        if let Event::PlayerMove(player_id, v) = e {
            if let Some(player_entity) = find_player_entity_mut(&mut state.entities, &player_id) {
              
              //  sub_step(&player_entity, &all_entities);

                /*  player_entity.1.pos += *v;
                
                // make collision check!
                for other_entity in state.entities.iter() {
                    collision_check(&player_entity, &other_entity)
                }*/
            }
    
        }
    }

    if is_server {
        // update non-players
    }
}