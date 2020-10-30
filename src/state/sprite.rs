#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sprite
{
    pub x:f32,
    pub y:f32,
    pub cols:f32,
    pub rows:f32
}

impl Default for Sprite 
{
    fn default() -> Self {
        Self {
            x:0.0,
            y:0.0,
            cols:16.0,
            rows:16.0
        }
    }
}