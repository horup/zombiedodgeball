use cgmath::Vector2;

#[derive(Debug, Copy, Clone)]
pub struct ClientData
{
    pub vel:Vector2<f32>,
    pub shoot:bool,
    pub client_id:u128
}

impl  Default for ClientData {
    fn default() -> Self {
        ClientData {
            vel:Vector2::new(0.0, 0.0),
            shoot:false,
            client_id:0
        }
    }
}