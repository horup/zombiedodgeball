use crate::{event::Event, world::World};


pub fn step(state:&mut World, is_server:bool, delta:f32, events:&mut Vec<Event>) 
{
    if is_server {
        // do cooldown of ability
        for e in state.entities.iter_mut() {
            if let Some(actor) = &mut e.actor {
                if actor.cooldown > 0.0 {
                    actor.cooldown = f32::max(0.0, actor.cooldown - delta);
                }
            }
        }

        for e in events.iter() {
            match e {
                Event::ShootAt(id, at) => {
                    
                },
                _ => {}
            }
        }
    }
}