use crate::data::State;
use crate::data::Event;
use super::{util::find_player_entity_mut};

pub fn physics(state:&mut State, is_server:bool, events:&[Event])
{
    // update players
    for e in events {
        if let Event::PlayerMove(player_id, v) = e {
            if let Some(player_entity) = find_player_entity_mut(&mut state.entities, &player_id) {
                player_entity.1.pos += *v;
                
                // make collision check!
            }
    
        }
    }

    if is_server {
        // update non-players
    }
}