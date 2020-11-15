use crate::{event::Event, world::World};


pub fn step<F:FnMut(Event)>(state:&mut World, is_server:bool, event:&Event, push_event:&mut F) 
{
    if is_server {
        match event {
            Event::ShootAt(my_id, at) => {
                if let Some(my_e) = state.entities.get_entity(*my_id) {
                }
            }
            Event::Tick(_, delta) => {
                for e in state.entities.iter_mut() {
                    if e.shooter.attached && e.shooter.cooldown > 0.0 {
                        e.shooter.cooldown = f32::max(e.shooter.cooldown - delta, 0.0);
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