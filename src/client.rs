use cgmath::{InnerSpace, Point2, Vector2};
use ggez::{Context, GameResult, event::{KeyCode, MouseButton}, graphics::{self, DrawParam, GlBackendSpec, ImageGeneric, Rect}, input::{keyboard, mouse}, timer};
use uuid::Uuid;

use crate::{event::Event, world::{Entity, World}};

struct Images {
    pub spritesheet:ImageGeneric<GlBackendSpec>
}
pub struct Client
{
    current:World,
    previous:World,
    images:Images,
    client_id:u128
}


#[derive(Copy, Clone, Debug)]
pub struct Input 
{
    pub dpad:Vector2<f32>,
    pub shoot:bool,
    pub mouse_world_pos:Point2<f32>
}

impl Default for Input
{
    fn default() -> Self {
        Self {
            dpad:Vector2::new(0.0, 0.0),
            shoot:false,
            mouse_world_pos:Point2::new(0.0, 0.0)
        }
    }
}

impl Client
{
    pub fn update(&mut self, new_state:World)
    {
        self.previous = self.current.clone();
        self.current = new_state;
    }

    pub fn new(ctx:&mut Context) -> Client
    {
        graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);

        let bytes = include_bytes!("./resources/spritesheet.png");
        let img = image::load_from_memory(bytes).unwrap();
        
        let spritesheet = img.to_rgba();
        let spritesheet = graphics::Image::from_rgba8(ctx, spritesheet.width() as u16, spritesheet.height() as u16, &spritesheet).unwrap();

        Self {
            current:World::new(),
            previous:World::new(),
            images:Images {
                spritesheet:spritesheet
            },
            client_id:Uuid::new_v4().as_u128()
        }
    }

    fn current_input(ctx:&mut Context, zoom:f32) -> Input
    {
        let mut input = Input::default();
        input.dpad.y = if keyboard::is_key_pressed(ctx, KeyCode::W) {-1.0} else {0.0};
        input.dpad.y = if keyboard::is_key_pressed(ctx, KeyCode::S) {1.0} else {input.dpad.y};
        input.dpad.x = if keyboard::is_key_pressed(ctx, KeyCode::A) {-1.0} else {0.0};
        input.dpad.x = if keyboard::is_key_pressed(ctx, KeyCode::D) {1.0} else {input.dpad.x};
        input.shoot = mouse::button_pressed(ctx, MouseButton::Left);
        let cursor = mouse::position(&ctx);
        input.mouse_world_pos = Point2::new(cursor.x / zoom, cursor.y / zoom);
        input
    }

    pub fn find_player_entity_mut(&mut self) -> Option<&mut Entity>
    {
        let client_id = self.client_id;
        self.current.entities.iter_mut().find(|e|{
            if let Some(player) = e.player {
                if player.client_id == client_id {
                    return true;
                }
            }

            false
        })
    }

    pub fn update_player(&mut self, delta:f32, input:&Input, events:&mut Vec<Event>)
    {
        let player_id = self.client_id;

        if let Some(e) = self.find_player_entity_mut() {
            let mut vel = input.dpad;
            vel = vel * 2.0 * delta;
            if vel.magnitude() > 0.0 {
                events.push(Event::ForceMovement(e.id, vel));
            }

            if input.shoot {
                events.push(Event::ShootAt(e.id, input.mouse_world_pos));
            }
            
        } else {
            if input.shoot {
                // no actor, I want to spawn instead
                let e = Event::PlayerSpawn(self.client_id);
                events.push(e);
            }
        }
    }
    

    pub fn render(&mut self, ctx:&mut Context, tick_rate_ps:u32) -> GameResult<Vec<Event>>
    {
        let mut events = Vec::new();
        let delta = timer::average_delta(ctx).as_secs_f32();
        //let res = self.update_player(delta);
    
        let remaining = timer::remaining_update_time(ctx);
        let alpha = remaining.as_secs_f32() as f32 * tick_rate_ps as f32;
      
        let size = graphics::drawable_size(ctx);
        let zoom = 32.0;
        let sprite_size = 16.0;
        let r = Rect {
            x:0.0,
            y:0.0,
            w:size.0 / zoom,
            h:size.1 / zoom
        };
        graphics::set_screen_coordinates(ctx, r)?;
        graphics::clear(ctx, graphics::BLACK);

        let input = Client::current_input(ctx, zoom);
        self.update_player(delta, &input, &mut events);

        let current = &self.current;
        let previous = &self.previous;

        for curr in current.entities.iter() {
            if let Some(prev) = previous.entities.get_entity(curr.id) {
                if let Some(sprite) = curr.sprite {
                    let v:Vector2<f32> = (curr.pos - prev.pos) * alpha;
                
                    let mut params = DrawParam::new();
                    params.dest.x = prev.pos.x + v.x - 0.5;
                    params.dest.y = prev.pos.y + v.y - 0.5;
                    params.src.x = sprite.x / sprite.cols;
                    params.src.y = sprite.y / sprite.rows;
                    params.src.w = 1.0 / sprite.cols;
                    params.src.h = 1.0 / sprite.rows;
                    params.scale.x = 1.0 / sprite_size;
                    params.scale.y = 1.0 / sprite_size;
                    graphics::draw(ctx,   &self.images.spritesheet, params)?;
                }
            }
        }

        graphics::present(ctx)?;
        
        Ok(events)
    }
}