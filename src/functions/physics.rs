use cgmath::{Point2, Vector2, prelude::*};
use collision::{Aabb2, prelude::*};
use gamestate::{Collection, ID};

use crate::data::{Entity, State};
use crate::data::Event;
use super::{util::{find_player_entity, find_player_entity_mut}};

#[derive(Copy, Clone)]
pub struct Body {
    pub id:ID,
    pub pos:Point2<f32>,
    pub vel:Vector2<f32>
}

impl Body {
    pub fn aabb2(&self) -> Aabb2<f32> {
        let r = 0.5;
        Aabb2::new(Point2::new(self.pos.x - r, self.pos.y - r), Point2::new(self.pos.x + r, self.pos.y + r))
    }
}

impl From<&Entity> for Body {
    fn from(e: &Entity) -> Self {
        Self {
            id:e.id,
            pos:e.pos,
            vel:e.vel
        }
    }
}

pub fn collision_check(entity:&Entity, other_entity:&Entity) -> bool
{
    let e1 = entity;
    let e2 = other_entity;
    e1.aabb2().intersects(&e2.aabb2())
}

pub fn sub_step(body:&mut Body, all_entities:&Collection<Entity>)
{
    let max = 0.1;
    let distance = body.vel.magnitude();
    let mut remaining = distance;
    while remaining > 0.0 {
        let step = if distance < max { distance} else {max};
        let dir = body.vel.normalize();
        let vs = [Vector2::new(0.0, dir.y * step), Vector2::new(dir.x * step, 0.0)];
        for v in vs.iter()
        {
            let body_before = *body;
            body.pos += *v;
            let mut collision = false;
            for e in all_entities.iter() {
                if e.id == body.id {
                    continue;
                }
                
                let other_body = Body::from(e);
                let v2:Vector2<f32> = body.pos - other_body.pos;
                let v2 = v2.normalize();
                if v.dot(v2) < 0.0 && body.aabb2().intersects(&other_body.aabb2())
                {
                    collision = true;
                    break;
                }
            }
    
            if collision {
                *body = body_before;
            }
        }

        remaining -= step;
    }
}

pub fn step(state:&mut State, is_server:bool, events:&[Event])
{
    let max_step = 0.1;
   // let all_entities = state.entities;
    // update players
    for e in events {
        if let Event::PlayerMove(player_id, v) = e {
            if let Some(player_entity) = find_player_entity(&state.entities, &player_id) {
                let mut body = Body::from(&*player_entity);
                body.vel = *v; 
                sub_step(&mut body, &state.entities);

                if let Some(player_entity) = state.entities.get_entity_mut(player_entity.id) {
                    player_entity.pos = body.pos;
                    player_entity.vel = body.vel;
                }
                //player_entity.pos = body.pos;
                //player_entity.vel = body.vel;

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