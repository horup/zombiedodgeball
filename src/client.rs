use cgmath::Vector2;
use ggez::{Context, GameResult, event::{KeyCode, MouseButton}, graphics::{self, DrawParam, GlBackendSpec, ImageGeneric, Rect}, input::{keyboard, mouse}, timer};

use crate::state::State;

struct Images {
    pub spritesheet:ImageGeneric<GlBackendSpec>
}
pub struct Client
{
    current:State,
    previous:State,
    images:Images
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
            }
        }
    }

    pub fn render(&mut self, ctx:&mut Context, tick_rate_ps:u32) -> GameResult<()>
    {
        let mut current = &mut self.current;
        current.input.dpad.y = if keyboard::is_key_pressed(ctx, KeyCode::W) {-1.0} else {0.0};
        current.input.dpad.y = if keyboard::is_key_pressed(ctx, KeyCode::S) {1.0} else {current.input.dpad.y};
        current.input.dpad.x = if keyboard::is_key_pressed(ctx, KeyCode::A) {-1.0} else {0.0};
        current.input.dpad.x = if keyboard::is_key_pressed(ctx, KeyCode::D) {1.0} else {current.input.dpad.x};
        current.input.shoot = mouse::button_pressed(ctx, MouseButton::Left);
        
        let remaining = timer::remaining_update_time(ctx);
        let alpha = remaining.as_secs_f32() as f32 * tick_rate_ps as f32;// / self.tick_rate_ps as f32;
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

		graphics::present(ctx)
    }
}