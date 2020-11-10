use cgmath::{Point2, Vector2, prelude::*};
use collision::{Aabb2, prelude::*};
use gamestate::{ID};

use crate::data::{Entity, State};
use crate::data::Event;
use super::{util::{find_player_entity}};

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

impl From<&mut Entity> for Body {
    fn from(e: &mut Entity) -> Self {
        Self {
            id:e.id,
            pos:e.pos,
            vel:e.vel
        }
    }
}

pub fn move_body(body:&mut Body, other_bodies:&[Body])
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
            for other_body in other_bodies.iter() {
                if other_body.id == body.id {
                    continue;
                }

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
    let all_bodies:Vec<Body> = state.entities.iter().map(|e| {
        Body::from(e)
    }).collect();

    // process events from players first!
    // players are assumed to have calculated their velocities them-seleves
    for e in events {
        if let Event::PlayerMove(player_id, v) = e {
            if let Some(player_entity) = find_player_entity(&state.entities, &player_id) {
                let mut body = Body::from(player_entity);
                body.vel = *v; 
                move_body(&mut body, &all_bodies);

                if let Some(player_entity) = state.entities.get_entity_mut(player_entity.id) {
                    player_entity.pos = body.pos;
                    player_entity.vel = body.vel;
                }
            }
    
        }
    }

    let all_bodies:Vec<Body> = state.entities.iter().map(|e| {
        Body {
            id:e.id,
            pos:e.pos,
            vel:e.vel
        }
    }).collect();

    if is_server {
        for e in state.entities.clone().iter() {
            // update non-players
            let mut body = Body::from(e);
            move_body(&mut body, &all_bodies);
            if let Some(e) = state.entities.get_entity_mut(body.id) {
                e.pos = body.pos;
                e.vel = body.vel;
            }
            
        }

    }
}