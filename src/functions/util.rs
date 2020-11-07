use gamestate::{Entities, EntityID};
use crate::data::Entity;


pub fn find_player_entity_mut<'a>(entities:&'a mut Entities<Entity>, player_id:&u128) -> Option<(EntityID, &'a mut Entity)>
{
    let player_entity = entities.iter_mut().find(|e| {
        if let Some(player) = e.1.player {
            if &player.client_id == player_id {
                return true;
            }
        }
        false
    });
    
    return player_entity;
}

pub fn find_player_entity<'a>(entities:&'a Entities<Entity>, player_id:&u128) -> Option<(EntityID, &'a Entity)>
{
    let player_entity = entities.iter().find(|e| {
        if let Some(player) = e.1.player {
            if &player.client_id == player_id {
                return true;
            }
        }
        false
    });
    
    return player_entity;
}