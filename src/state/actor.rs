
#[derive(Copy, Clone, PartialEq)]
pub struct Actor
{
    pub speed:f32
}

impl Default for Actor
{
    fn default() -> Self {
        Self {
            speed:1.0
        }
    }
}
