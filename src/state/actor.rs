
#[derive(Copy, Clone, PartialEq)]
pub enum Actor
{
    None,
    Zombie(Common, Zombie),
    Player(Common, Player)
}

#[derive(Copy, Clone, PartialEq)]
pub struct Common
{
    pub speed:f32
}

impl Default for Common
{
    fn default() -> Self {
        Self {
            speed:1.0
        }
    }
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