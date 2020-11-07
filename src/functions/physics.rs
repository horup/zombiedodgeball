use cgmath::{Point2, Vector2};
use collision::{Aabb2, prelude::*};
use gamestate::EntityID;

use crate::data::{Entity, State};
use crate::data::Event;
use super::{util::find_player_entity_mut};

pub fn collision_check(entity:&(EntityID, &Entity), other_entity:&(EntityID, &Entity)) -> bool
{
    let e1 = entity.1;
    let e2 = other_entity.1;
    e1.aabb2().intersects(&e2.aabb2())
}

pub fn step(state:&mut State, is_server:bool, events:&[Event])
{
    let max_step = 0.1;
    // update players
    for e in events {
        if let Event::PlayerMove(player_id, v) = e {
            if let Some(player_entity) = find_player_entity_mut(&mut state.entities, &player_id) {
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