use cgmath::{Point2, Vector2};
use cgmath::prelude::*;
use collision::{Aabb2, prelude::*};
use gamestate::EntityID;

use crate::{ClientData, state::{Actor, Dodgeball, Player, Sprite, State}};

pub fn update_spawn(state:&mut State, iterations:i32)
{
    if iterations % 20 == 0 {
        let (id, e) = state.entities.new_entity_replicated().expect("could not create entity");
        e.pos = Vector2::new(rand::random::<f32>() * 20.0, rand::random::<f32>() * 20.0);
        e.sprite = Some(Sprite {
            x:1.0,
            ..Sprite::default()
        });
        e.actor = Some(Actor {
            speed:1.0,
            ..Actor::default()
        });
    }
}

pub fn update_clients(state:&mut State, delta:f32, client_data:&[ClientData])
{
    for cd in client_data
    {
        let client_id = cd.client_id;
        let player_entity = state.entities.iter_mut().find(|(id, e)| {
            if let Some(p) = e.player {
                return client_id == p.client_id;
            }
            false
        });

        if cd.shoot {
            if let Some((_, player_entity)) = player_entity {
                // player entity exist
                let mut shoot = false;
                if let Some(actor) = &mut player_entity.actor {
                    if actor.cooldown <= 0.0 {
                        actor.cooldown = actor.max_cooldown;
                        shoot = true;
                    }
                }

                let player_entity = *player_entity; // copy value as to avoid mutable borrow
                if shoot {
                    let (id, dodge_ball_entity) = state.entities.new_entity_replicated().expect("could not spawn ball");
                    dodge_ball_entity.pos = player_entity.pos;
                    dodge_ball_entity.vel = cd.shoot_at - dodge_ball_entity.pos;
                    dodge_ball_entity.vel = dodge_ball_entity.vel.normalize() * 15.0;
                    dodge_ball_entity.sprite = Some(Sprite {
                        x:2.0,
                        ..Sprite::default()
                    });
                    dodge_ball_entity.dodgeball = Some(Dodgeball {

                    });
                }
            }
            else {
                // player entity does not exist, spawn him
                let (id, e) = state.entities.new_entity_replicated().expect("could not player entity");
                e.pos = Vector2::new(7.0, 15.0);
                e.actor = Some(Actor {
                    speed:1.0,
                    ..Actor::default()
                });
                e.player = Some(Player {client_id:client_id});
                e.sprite = Some(Sprite::default());
                println!("spawning player entity {:?}", id);
            }
        }

        for (_, e) in state.entities.iter_mut() {
            if let Some(p) = e.player {
                if p.client_id == client_id
                {
                    e.vel += cd.vel;
                    //e.pos += cd.vel;
                }
            }
        }
    }
}

pub fn update_actors(state:&mut State, delta:f32)
{
    for (_, e) in state.entities.iter_mut() {
        if let Some(actor) = &mut e.actor {
            if actor.cooldown > 0.0 {
                actor.cooldown -= delta;
                if actor.cooldown < 0.0 {
                    actor.cooldown = 0.0;
                }
            }
        } 
    }
}

#[derive(Copy, Clone, Default)]
pub struct CollisionResult
{
    pub entity_id:EntityID,
    pub other_entity_id:EntityID
}

pub fn update_dodge_ball(state:&mut State, delta:f32, collisions:&Vec<CollisionResult>)
{
    for col in collisions.iter()
    {
        if let Some((id, e)) = state.entities.get_entity_mut(col.entity_id)
        {
            if e.dodgeball != None {
                state.entities.delete_entity(col.entity_id);
                state.entities.delete_entity(col.other_entity_id);
            }
        }
    }
}

pub fn update_movement(state:&mut State, delta:f32) -> Vec<CollisionResult>
{
    let mut collisions = Vec::new();
    let entitites = state.entities.clone();
    for (my_id,my_e) in state.entities.iter_mut() {
        let my_v = my_e.vel * delta;
        my_e.pos += my_v;

        let r = 0.5;
        let my_aabb = Aabb2::new(Point2 {x:my_e.pos.x - r, y:my_e.pos.y - r}, Point2 {x:my_e.pos.x + r, y:my_e.pos.y + r});

        for (other_id, other_e) in entitites.iter() {
            if my_id == other_id {
                continue;
            }

            let v = other_e.pos - my_e.pos;
            let d = my_v.dot(v);
            if d > 0.0 {
                let other_aabb = Aabb2::new(Point2 {x:other_e.pos.x - r, y:other_e.pos.y - r}, Point2 {x:other_e.pos.x + r, y:other_e.pos.y + r});
                if my_aabb.intersects(&other_aabb) {
                    my_e.vel = Vector2::new(0.0, 0.0);

                    collisions.push(CollisionResult {
                        entity_id:my_id,
                        other_entity_id:other_id
                    });
                }
            }

        }
    }

    return collisions;
}