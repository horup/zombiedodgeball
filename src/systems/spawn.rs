use cgmath::Vector2;

use crate::{state::{Actor, Player, Sprite, State}};

pub type ClientIDsWishingToJoin = [u128];

pub fn spawn(state:&mut State, delta:f32, iterations:i32, wish_to_join:&ClientIDsWishingToJoin)
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

    for client_id in wish_to_join
    {
        let player_entity = state.entities.iter_mut().find(|(id, e)| {
            if let Some(p) = e.player {
                return client_id == &p.client_id;
            }
            false
        });

        if player_entity == None {
                // player entity does not exist, spawn him
                let (id, e) = state.entities.new_entity_replicated().expect("could not player entity");
                e.pos = Vector2::new(7.0, 15.0);
                e.actor = Some(Actor {
                    speed:1.0,
                    ..Actor::default()
                });
                e.player = Some(Player {client_id:*client_id});
                e.sprite = Some(Sprite::default());
                println!("spawning player entity {:?}", id);
        }
    }
}