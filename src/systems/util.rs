use gamestate::Collection;
use crate::data::Entity;


pub fn find_player_entity_mut<'a>(entities:&'a mut Collection<Entity>, player_id:&u128) -> Option<&'a mut Entity>
{
    let player_entity = entities.iter_mut().find(|e| {
        if let Some(player) = e.player {
            if &player.client_id == player_id {
                return true;
            }
        }
        false
    });
    
    return player_entity;
}

pub fn find_player_entity<'a>(entities:&'a Collection<Entity>, player_id:&u128) -> Option<&'a Entity>
{
    let player_entity = entities.iter().find(|e| {
        if let Some(player) = e.player {
            if &player.client_id == player_id {
                return true;
            }
        }
        false
    });
    
    return player_entity;
}