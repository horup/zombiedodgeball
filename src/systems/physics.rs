use gamestate::EntityID;
use crate::{ClientData, state::State};


pub struct Collision {
    pub id:EntityID,
    pub other:EntityID
}

pub type OnCollision = fn(&mut State, Collision);

pub fn physics(state:&mut State, client_data:&[ClientData], collisions:&[OnCollision])
{

}