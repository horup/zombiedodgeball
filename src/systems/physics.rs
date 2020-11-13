use cgmath::{prelude::*, Point2, Vector2};
use collision::{prelude::*, Aabb2};
use gamestate::ID;

use crate::data::{Entity, Event, State};

fn aabb2(e: &Entity) -> Aabb2<f32> {
    let r = 0.5;
    Aabb2::new(
        Point2::new(e.pos.x - r, e.pos.y - r),
        Point2::new(e.pos.x + r, e.pos.y + r),
    )
}
/*
fn move_body(state:&mut State, events:&mut Vec<Event>)
{
    let max = 0.1;
    let distance = body.vel().magnitude();
    let mut remaining = distance;
    while remaining > 0.0 {
        let step = if distance < max { distance} else {max};
        let dir = body.vel().normalize();
        let vs = [Vector2::new(0.0, dir.y * step), Vector2::new(dir.x * step, 0.0)];
        for v in vs.iter()
        {
            let pos = *body.pos();
            *body.pos_mut() += *v;
            let mut collision = false;
            for other_body in other_bodies.clone() {
                if other_body.id() == body.id() {
                    continue;
                }

                let v2:Vector2<f32> = body.pos() - other_body.pos();
                let v2 = v2.normalize();
                if v.dot(v2) < 0.0 && body.aabb2().intersects(&other_body.aabb2())
                {
                    collision = true;
                    result.push(PhysicsEvent::CollisionWithEntityEvent(CollisionWithEntity {
                        entity_id:body.id(),
                        other_entity_id:other_body.id()
                    }));
                    break;
                }
            }

            if collision {
                *body.pos_mut() = pos;
            }
        }

        remaining -= step;
    }
}*/

pub fn step(state: &mut State, is_server: bool, events: &Vec<Event>) -> Vec<Event> {
    for event in events {
        match event {
            Event::ForceMovement(id, diff) => {

            }
            Event::Tick(iterations, delta) => {
                
            }
            _ => {}
        }
    }
    return Vec::new();
}

/*
let mut result = Vec::new();
for e in events {
    match e {
        PhysicsEvent::ForceMovementEvent(entity_id, diff) => {
            let range = 0..bodies.len();
            for i in range {
                if let Some(b) = bodies.get(i) {
                    if b.id() != entity_id {
                        continue;
                    }
                }
                let (left, right) = bodies.split_at_mut(i);
                if let Some((body, right)) = right.split_first_mut() {
                    let other_bodies = left.iter().chain(right.iter());
                    let org = *body.vel();
                    *body.vel_mut() = diff;
                    move_body(*body, &other_bodies.map(|x| &**x), &mut result);
                    *body.vel_mut() = org;
                }
            }
        },
        _ => {}
    }
}

if is_server {

    let range = 0..bodies.len();
    for i in range {
        let (left, right) = bodies.split_at_mut(i);
        if let Some((body, right)) = right.split_first_mut() {
            let other_bodies = left.iter().chain(right.iter());
            move_body(*body, &other_bodies.map(|x| &**x), &mut result);
        }
    }
}

return result;
*/
