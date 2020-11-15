use cgmath::{prelude::*, Point2, Vector2};
use collision::{prelude::*, Aabb2};

use crate::{world::Entity, event::Event, world::World};

fn aabb2(pos:&Point2<f32>) -> Aabb2<f32> {
    let r = 0.5;
    Aabb2::new(
        Point2::new(pos.x - r, pos.y - r),
        Point2::new(pos.x + r, pos.y + r),
    )
}

fn compute_movement<F:FnMut(Event)>(entity:&Entity, diff:&Vector2<f32>, state:&World, push_event:&mut F) -> Point2<f32>
{
    let mut res = entity.pos;
    let max = 0.1;
    let distance = diff.magnitude();
    if distance <= 0.0 {
        return res;
    }

    let mut remaining = distance;
    while remaining > 0.0 {
        let step = if distance < max { distance} else {max};
        let dir = diff.normalize();
        let vs = [Vector2::new(0.0, dir.y * step), Vector2::new(dir.x * step, 0.0)];
        let mut collision_occured = false;

        for v in vs.iter()
        {
            let mut p:Point2<f32> = res + v;
            if let Some(collision) = &entity.collision {
                for other_entity in state.entities.iter().filter(|e| e.id != entity.id && e.collision != None) {
                    let other_collision = &other_entity.collision.unwrap();
                    let v2 = res - other_entity.pos;
                    let v2 = v2.normalize();
                    if v.dot(v2) < 0.0 {
                        
                        if aabb2(&p).intersects(&aabb2(&other_entity.pos)) {
                            collision_occured = true;
                            push_event(Event::Collision(entity.id, other_entity.id));
                            break;
                        } 
                    }
                }
            }
            if !collision_occured {
                res = p;
            }
        }

        if collision_occured {
            break;
        }

        remaining -= step;
    }

    res
}


pub fn step<F:FnMut(Event)>(world: &mut World, is_server: bool, event:&Event, push_event:&mut F) {
    match event {
        Event::ForceMovement(id, diff) => {
            if let Some(e) = world.entities.get_entity(*id) {
                let res = compute_movement(e, diff, world, push_event);
                

                if let Some(e) = world.entities.get_entity_mut(*id) {
                    e.pos = res;
                }
            }
        }
        Event::Tick(_, delta) => {

            // fixme can be improved without clone I believe
            for e in world.entities.clone().iter() {
                let v = e.vel * *delta;
                let res = compute_movement(e, &v, world, push_event);
                if let Some(e) = world.entities.get_entity_mut(e.id) {
                    e.pos = res;
                }
            }
        }
        _ => {}
    }
}
