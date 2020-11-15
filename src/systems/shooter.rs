use crate::{event::Event, world::World};


pub fn step<F:FnMut(Event)>(state:&mut World, is_server:bool, event:&Event, push_event:&mut F) 
{
    if is_server {
        match event {
            Event::ShootAt(my_id, at) => {
                if let Some(my_e) = state.entities.get_entity(*my_id) {
                    if my_e.missile.attached {

                    }
                }
            }
            Event::Tick(_, delta) => {
                // do cooldown of ability
                for e in state.entities.iter_mut() {
                    if let Some(actor) = &mut e.actor {
                        if actor.cooldown > 0.0 {
                            actor.cooldown = f32::max(0.0, actor.cooldown - delta);
                        }
                    }
                }
            }
            Event::Collision(id1, id2) => {
                if let Some((e1, e2)) = state.entities.get_entity_pair(*id1, *id2)
                {
                    if e1.missile.attached {
                        push_event(Event::DeleteEntity(e1.id));
                        push_event(Event::DeleteEntity(e2.id));
                    }
                }
            },
            _ => {}
        }
    }
}