use crate::{event::Event, world::World};


pub fn step<F:FnMut(Event)>(world:&mut World, is_server:bool, event:&Event, push_event:&mut F) 
{
    if is_server {
        match event {
            Event::ShootAt(my_id, at) => {
                if let Some(my_e) = world.entities.get_entity_mut(*my_id) {

                    if let Some(shooter) = &mut my_e.shooter { 
                        if shooter.cooldown == 0.0 {
                            shooter.cooldown = 1.0;
                            let pos = my_e.pos;
                            world.spawn_missile(pos, *at);
                        }
                    }
                }
            }
            Event::Tick(_, delta) => {
                for e in world.entities.iter_mut() {
                    if let Some(shooter) = &mut e.shooter {
                        shooter.cooldown = f32::max(shooter.cooldown - delta, 0.0);
                    }
                }
            }
            Event::Collision(id1, id2) => {
                if let Some((e1, e2)) = world.entities.get_entity_pair(*id1, *id2)
                {
                    if let Some(missile) = e1.missile {
                        push_event(Event::DeleteEntity(e1.id));
                        push_event(Event::DeleteEntity(e2.id));
                    }
                }
            },
            _ => {}
        }
    }
}