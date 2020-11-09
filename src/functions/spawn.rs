use cgmath::{Point2, Vector2};

use crate::{data::{Actor, Event, Player, Sprite, State}};



pub fn step(state:&mut State, delta:f32, iterations:i32, events:&[Event])
{
    if iterations % 20 == 0 {
        let e = state.entities.new_entity_replicated().expect("could not create entity");
        e.pos = Point2::new(rand::random::<f32>() * 20.0, rand::random::<f32>() * 20.0);
        e.sprite = Some(Sprite {
            x:1.0,
            ..Sprite::default()
        });
        e.actor = Some(Actor {
            speed:1.0,
            ..Actor::default()
        });
    }

    
    for e in events.iter() {
        if let Event::PlayerSpawn(client_id) = e {
            let player_entity = state.entities.iter_mut().find(|e| {
                if let Some(p) = e.player {
                    return client_id == &p.client_id;
                }
                false
            });

            if player_entity == None {
                    // player entity does not exist, spawn him
                    let e = state.entities.new_entity_replicated().expect("could not player entity");
                    e.pos = Point2::new(7.0, 15.0);
                    e.actor = Some(Actor {
                        speed:1.0,
                        ..Actor::default()
                    });
                    e.player = Some(Player {client_id:*client_id});
                    e.sprite = Some(Sprite::default());
                    println!("spawning player entity {:?}", e.id);
            }
        }
    }
}