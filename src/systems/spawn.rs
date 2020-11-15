use cgmath::{Point2, Vector2};

use crate::{event::Event, components::Shooter, world::World};
use crate::components::{Sprite, Player};


pub fn step<F:FnMut(Event)>(state:&mut World, is_server:bool, event:&Event, push_event:&mut F)
{
    match event {
        Event::Tick(iterations, delta) => {
            if iterations % 20 == 0 {
                let e = state.entities.new_entity_replicated().expect("could not create entity");
                e.pos = Point2::new(rand::random::<f32>() * 20.0, rand::random::<f32>() * 20.0);
                e.sprite = Some(Sprite {
                    x:1.0,
                    ..Sprite::default()
                });
                e.vel.x = 1.0;
            }
        },
        Event::PlayerSpawn(player_id) => {
            let player_entity = state.entities.iter_mut().find(|e| {
                if let Some(p) = e.player {
                    return player_id == &p.client_id;
                }
                false
            });

            if player_entity == None {
                    // player entity does not exist, spawn him
                    let e = state.entities.new_entity_replicated().expect("could not player entity");
                    e.pos = Point2::new(7.0, 15.0);
                    e.shooter = Some(Shooter {
                        attached:true,
                        cooldown:0.0
                    });
                    e.player = Some(Player {client_id:*player_id, ..Player::default()});
                    e.sprite = Some(Sprite::default());
                    println!("spawning player entity {:?}", e.id);
            }
        },
        _ => {}
    }
}