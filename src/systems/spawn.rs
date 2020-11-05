use cgmath::Vector2;

use crate::state::{Actor, Sprite, State};

pub fn spawn(state:&mut State, iterations:i32)
{
    if iterations % 20 == 0 {
        let (id, e) = state.entities.new_entity_replicated().expect("could not create entity");
        e.pos = Vector2::new(rand::random::<f32>() * 20.0, rand::random::<f32>() * 20.0);
        e.sprite = Some(Sprite {
            x:1.0,
            ..Sprite::default()
        });
        e.actor = Some(Actor {
            speed:1.0,
            ..Actor::default()
        });
    }
}