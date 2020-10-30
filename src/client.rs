use cgmath::Vector2;
use gamestate::EntityID;
use ggez::{Context, GameResult, event::{KeyCode, MouseButton}, graphics::{self, DrawParam, GlBackendSpec, ImageGeneric, Rect}, input::{keyboard, mouse}, timer};
use uuid::Uuid;

use crate::state::{Actor, Entity, State};

struct Images {
    pub spritesheet:ImageGeneric<GlBackendSpec>
}
pub struct Client
{
    current:State,
    previous:State,
    images:Images,
    input:Input,
    client_id:u128
}


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

impl Client
{
    pub fn update(&mut self, new_state:State)
    {
        self.previous = self.current.clone();
        self.current = new_state;
    }

    pub fn new(ctx:&mut Context) -> Client
    {
        let bytes = include_bytes!("./resources/spritesheet.png");
        let img = image::load_from_memory(bytes).unwrap();
        
        let spritesheet = img.to_rgba();
        let spritesheet = graphics::Image::from_rgba8(ctx, spritesheet.width() as u16, spritesheet.height() as u16, &spritesheet).unwrap();

        Self {
            current:State::new(),
            previous:State::new(),
            images:Images {
                spritesheet:spritesheet
            },
            input:Input::default(),
            client_id:Uuid::new_v4().as_u128()
        }
    }

    pub fn update_input(&mut self, ctx:&mut Context)
    {
        self.input.dpad.y = if keyboard::is_key_pressed(ctx, KeyCode::W) {-1.0} else {0.0};
        self.input.dpad.y = if keyboard::is_key_pressed(ctx, KeyCode::S) {1.0} else {self.input.dpad.y};
        self.input.dpad.x = if keyboard::is_key_pressed(ctx, KeyCode::A) {-1.0} else {0.0};
        self.input.dpad.x = if keyboard::is_key_pressed(ctx, KeyCode::D) {1.0} else {self.input.dpad.x};
        self.input.shoot = mouse::button_pressed(ctx, MouseButton::Left);
    }

    pub fn find_player_entity_mut(&mut self) -> Option<(EntityID, &mut Entity)>
    {
        let client_id = self.client_id;
        self.current.entities.iter_mut().find(|(_, e)|{
            if let Actor::Player(_, player) = e.actor {
                if player.client_id == client_id {
                    return true;
                }
            }

            false
        })
    }

    pub fn update_player(&mut self, delta:f32) -> ClientData
    {
        let mut data = ClientData::default();
        data.client_id = self.client_id;
        if let Some((id, entity)) = self.find_player_entity_mut() {
            if let Actor::Player(common, player) = entity.actor {
                let mut vel = self.input.dpad;
                vel = vel * delta;
                data.vel = vel * common.speed;
            }
        };

        data.shoot = self.input.shoot;
        data
    }

    pub fn render(&mut self, ctx:&mut Context, tick_rate_ps:u32) -> GameResult<ClientData>
    {
        let delta = timer::average_delta(ctx).as_secs_f32();
        self.update_input(ctx);
        let res = self.update_player(delta);
    
        let remaining = timer::remaining_update_time(ctx);
        let alpha = remaining.as_secs_f32() as f32 * tick_rate_ps as f32;
        let current = &self.current;
        let previous = &self.previous;
        let size = graphics::size(ctx);
        let zoom = 32.0;
        let sprite_size = 16.0;
        graphics::set_screen_coordinates(ctx, Rect {
            x:0.0,
            y:0.0,
            w:size.0 / zoom,
            h:size.1 / zoom
        })?;
        graphics::clear(ctx, graphics::BLACK);

        for (id, curr) in current.entities.iter()
        {
            if let Some((_, prev)) = previous.entities.get_entity(id)
            {
                let v:Vector2<f32> = (curr.pos - prev.pos) * alpha;
                
                let mut params = DrawParam::new();
                params.dest.x = prev.pos.x + v.x;
                params.dest.y = prev.pos.y + v.y;
                params.scale.x = 1.0 / sprite_size;
                params.scale.y = 1.0 / sprite_size;
                graphics::draw(ctx,   &self.images.spritesheet, params)?;
            }
        }

        graphics::present(ctx)?;
        
        Ok(res)
    }
}