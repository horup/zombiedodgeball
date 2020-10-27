use std::sync::mpsc;

use ggez::{Context, ContextBuilder, GameResult, event::KeyCode, event::MouseButton, graphics::DrawParam, graphics::{self, GlBackendSpec, ImageGeneric}, input::{keyboard, mouse}, timer};
use ggez::event::{self, EventHandler};
mod state;
use state::State;

mod render;
mod server;
mod msg;
pub use server::*;
pub use msg::*;

pub fn main() {

    let server = Server::start(server_rx, client_tx);

    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
		.expect("aieee, could not create ggez context!");
    let mut zombie_dodge_ball = ZombieDodgeBall::new(&mut ctx);
    match event::run(&mut ctx, &mut event_loop, &mut zombie_dodge_ball) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }

    server.join().expect("could not join");
}

struct Images {
    pub spritesheet:ImageGeneric<GlBackendSpec>
}
struct ZombieDodgeBall {
    pub current:State,
    pub previous:State,
    pub images:Images,
    pub tick_rate_ps:u32
}

impl ZombieDodgeBall {
    pub fn new(ctx: &mut Context) -> ZombieDodgeBall {
        let bytes = include_bytes!("./resources/spritesheet.png");
        let img = image::load_from_memory(bytes).unwrap();
        
        let spritesheet = img.to_rgba();
        let spritesheet = graphics::Image::from_rgba8(ctx, spritesheet.width() as u16, spritesheet.height() as u16, &spritesheet).unwrap();

        ZombieDodgeBall {
            current: State::new(),
            previous:State::new(),
            images: Images {
                spritesheet:spritesheet,
            },
            tick_rate_ps:20
		}
    }
}

impl EventHandler for ZombieDodgeBall {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
      
        let mut current = &mut self.current;
        current.input.dpad.y = if keyboard::is_key_pressed(ctx, KeyCode::W) {-1.0} else {0.0};
        current.input.dpad.y = if keyboard::is_key_pressed(ctx, KeyCode::S) {1.0} else {current.input.dpad.y};
        current.input.dpad.x = if keyboard::is_key_pressed(ctx, KeyCode::A) {-1.0} else {0.0};
        current.input.dpad.x = if keyboard::is_key_pressed(ctx, KeyCode::D) {1.0} else {current.input.dpad.x};
        current.input.shoot = mouse::button_pressed(ctx, MouseButton::Left);
        
        if timer::check_update_time(ctx, self.tick_rate_ps){
            self.previous = current.clone();
            current.update();
        }

		Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let remaining = timer::remaining_update_time(ctx);
        let alpha = remaining.as_secs_f32() as f32 * self.tick_rate_ps as f32;// / self.tick_rate_ps as f32;
       
        self.render(ctx, alpha)?;
        Ok(())
    }
}