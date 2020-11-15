use cgmath::{Point2};

use crate::{event::Event, world::World};


pub fn step<F:FnMut(Event)>(world:&mut World, is_server:bool, event:&Event, push_event:&mut F)
{
    match event {
        Event::Tick(iterations, _) => {
            if iterations % 20 == 0 {
                world.spawn_zombie(Point2::new(rand::random::<f32>() * 20.0, rand::random::<f32>() * 20.0));
            }
        },
        Event::PlayerSpawn(player_id) => {
            let player_entity = world.entities.iter_mut().find(|e| {
                if let Some(p) = e.player {
                    return player_id == &p.client_id;
                }
                false
            });

            if player_entity == None {
                    world.spawn_player(Point2::new(7.0, 15.0), *player_id);
            }
        },
        _ => {}
    }
}