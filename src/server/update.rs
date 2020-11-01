use cgmath::Vector2;
use gamestate::EntityID;

use crate::{ClientData, state::{Actor, Player, Sprite, State}};


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
                    dodge_ball_entity.vel = Vector2::new(1.0, 1.0);
                    dodge_ball_entity.sprite = Some(Sprite {
                        x:2.0,
                        ..Sprite::default()
                    })
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
                    e.pos += cd.vel;
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

pub fn update_movement(state:&mut State, delta:f32)
{
    for (_,e) in state.entities.iter_mut() {
        e.pos += e.vel * delta;
    }
}