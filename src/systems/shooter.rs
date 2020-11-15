use crate::{components::Missile, event::Event, components::Sprite, world::World};
use cgmath::prelude::*;


pub fn step<F:FnMut(Event)>(state:&mut World, is_server:bool, event:&Event, push_event:&mut F) 
{
    if is_server {
        match event {
            Event::ShootAt(my_id, at) => {
                if let Some(my_e) = state.entities.get_entity_mut(*my_id) {

                    if let Some(shooter) = &mut my_e.shooter { 
                        if shooter.cooldown == 0.0 {
                            shooter.cooldown = 1.0;
                            let pos = my_e.pos;
                            if let Some(new_e) = state.entities.new_entity_replicated() {
                                new_e.missile = Some(Missile {
                                });
                                new_e.sprite = Some(Sprite {
                                    x:2.0,
                                    ..Sprite::default()
                                });
                                let v = at - pos;
                                let v = v.normalize() * 10.0;
                                new_e.pos = pos;
                                new_e.vel = v;
                            }
                        }
                    }
                }
            }
            Event::Tick(_, delta) => {
                for e in state.entities.iter_mut() {
                    if let Some(shooter) = &mut e.shooter {
                        shooter.cooldown = f32::max(shooter.cooldown - delta, 0.0);
                    }
                }
            }
            Event::Collision(id1, id2) => {
                if let Some((e1, e2)) = state.entities.get_entity_pair(*id1, *id2)
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