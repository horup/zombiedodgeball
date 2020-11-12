use cgmath::{Point2, Vector2, prelude::*};
use collision::{Aabb2, prelude::*};
use gamestate::{ID};

pub trait PhysicsBody : gamestate::Entity
{
    fn pos(&self) -> &Point2<f32>;
    fn pos_mut(&mut self) -> &mut Point2<f32>;
    fn vel(&self) -> &Vector2<f32>;
    fn vel_mut(&mut self) -> &mut Vector2<f32>;

    fn is_player(&self) -> bool
    {
        false
    }
    fn aabb2(&self) -> Aabb2<f32>
    {
        let r = 0.5;
        Aabb2::new(Point2::new(self.pos().x - r, self.pos().y - r), Point2::new(self.pos().x + r, self.pos().y + r))
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PhysicsEvent
{
    ForceMovementEvent(ID, Vector2<f32>)
}


fn move_body<'a, T:PhysicsBody + 'a, I:IntoIterator<Item = &'a T> + Clone>(body:&mut T, other_bodies:&I)//left:&[&mut T], right:&[&mut T])
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
                    break;
                }
            }
    
            if collision {
                *body.pos_mut() = pos;
            }
        }

        remaining -= step;
    }
}

pub fn step<T:PhysicsBody, I:IntoIterator<Item = PhysicsEvent>>(bodies:&mut [&mut T], is_server:bool, events:I)
{
    /*let all_bodies:Vec<PhysicsBody> = state.entities.iter().map(|e| {
        PhysicsBody::from(e)
    }).collect();*/

    // process events from players first!
    // players are assumed to have calculated their velocities them-seleves
   /* for e in events {
        if let Event::PlayerMove(player_id, v) = e {
            if let Some(player_entity) = find_player_entity(&state.entities, &player_id) {
                let mut body = PhysicsBody::from(player_entity);
                body.vel = *v; 
                move_body(&mut body, &all_bodies);

                if let Some(player_entity) = state.entities.get_entity_mut(player_entity.id) {
                    player_entity.pos = body.pos;
                    player_entity.vel = body.vel;
                }
            }
    
        }
    }

    let all_bodies:Vec<PhysicsBody> = state.entities.iter().map(|e| {
        PhysicsBody {
            id:e.id,
            pos:e.pos,
            vel:e.vel
        }
    }).collect();*/

    if is_server {
        /*for i in 0..bodies.len() {
            if let Some(body) = bodies.get(i) {
                let mut clone = **body;
                move_body(&mut clone, bodies);
                if let Some(body) = bodies.get_mut(i) {
                    **body = clone;
                }
            }
        }*/

        for e in events {
            match e {
                PhysicsEvent::ForceMovementEvent(entity_id, diff) => {
                    let range = 0..bodies.len();
                    for i in range {
                        let (left, right) = bodies.split_at_mut(i);
                        if let Some((body, right)) = right.split_first_mut() {
                            let other_bodies = left.iter().chain(right.iter());
                            let org = *body.vel();
                            *body.vel_mut() = diff;
                            move_body(*body, &other_bodies.map(|x| &**x));
                            *body.vel_mut() = org;
                        }
                    }
                },
                _ => {}
            }
        }

        let range = 0..bodies.len();
        for i in range {
            let (left, right) = bodies.split_at_mut(i);
            if let Some((body, right)) = right.split_first_mut() {
                let other_bodies = left.iter().chain(right.iter());
                move_body(*body, &other_bodies.map(|x| &**x));
            }
        }

       /* for i in 0..bodies.len() {
            let rest:Vec<&Body> = bodies.get
            let left = &bodies[0..i];
            let right = &bodies[i..bodies.len()];
            let m = bodies.get_mut(i).unwrap();
            move_body(*m, &*left);
        }*/
        /*for i in 0..bodies.len() {
            if let Some(body) = bodies.get_mut(i) {
                move_body(*body, &*bodies);
            }
            /*let mut body = &mut bodies[i];
            for other_body in bodies {
                body.pos_mut().x = 0.0;
            }*/
        }*/
       /* for e in bodies.iter() {
            //let split = bodies.split(|x| x.id() != e.id());
            //e.pos_mut().x=1.0;
            // update non-players

           /* let mut body = PhysicsBody::from(e);
            move_body(&mut body, &all_bodies);
            if let Some(e) = state.entities.get_entity_mut(body.id) {
                e.pos = body.pos;
                e.vel = body.vel;
            }*/
            
        }*/

    }
}