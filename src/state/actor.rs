
#[derive(Copy, Clone, PartialEq)]
pub enum Actor
{
    None,
    Zombie(Zombie),
    Player(Player)
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Player
{
    pub client_id:u128
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Zombie
{

}