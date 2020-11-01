
#[derive(Copy, Clone, PartialEq)]
pub struct Actor
{
    pub speed:f32,
    pub cooldown:f32,
    pub max_cooldown:f32
}

impl Default for Actor
{
    fn default() -> Self {
        Self {
            speed:1.0,
            cooldown:0.0,
            max_cooldown:1.0
        }
    }
}
