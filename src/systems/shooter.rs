use crate::data::State;

pub fn step(state:&mut State, is_server:bool, delta:f32) 
{
    if is_server {
        for e in state.entities.iter_mut() {
            if let Some(actor) = &mut e.actor {
                if actor.cooldown > 0.0 {
                    actor.cooldown = f32::max(0.0, actor.cooldown - delta);
                }
            }
        }
    }
}