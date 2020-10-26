use cgmath::{Vector2};

#[derive(Copy, Clone, Debug)]
pub struct Input 
{
    pub dpad:Vector2<f32>,
    pub shoot:bool
}

impl Default for Input
{
    fn default() -> Self {
        Self {
            dpad:Vector2::new(0.0, 0.0),
            shoot:false
        }
    }
}