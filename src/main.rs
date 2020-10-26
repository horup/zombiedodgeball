use ggez::{Context, ContextBuilder, graphics::DrawParam, GameResult, graphics::{self, GlBackendSpec, ImageGeneric}};
use ggez::event::{self, EventHandler};
mod state;
use state::State;
use ggez::context::*;
pub fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
		.expect("aieee, could not create ggez context!");
    let mut zombie_dodge_ball = ZombieDodgeBall::new(&mut ctx);
    match event::run(&mut ctx, &mut event_loop, &mut zombie_dodge_ball) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct Images {
    pub spritesheet:ImageGeneric<GlBackendSpec>
}
struct ZombieDodgeBall {
    pub current:State,
    pub images:Images
}

impl ZombieDodgeBall {
    pub fn new(ctx: &mut Context) -> ZombieDodgeBall {
        let bytes = include_bytes!("./resources/spritesheet.png");
        let img = image::load_from_memory(bytes).unwrap();
        
        let spritesheet = img.to_rgba();
        let spritesheet = graphics::Image::from_rgba8(ctx, spritesheet.width() as u16, spritesheet.height() as u16, &spritesheet).unwrap();


        ZombieDodgeBall {
            current: State::new(),
            images: Images {
                spritesheet:spritesheet
            }
		}
    }
}

impl EventHandler for ZombieDodgeBall {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let keyboard = &ctx.keyboard_context;
		Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        graphics::draw(ctx,   &self.images.spritesheet, DrawParam::new())?;
		graphics::present(ctx)
    }
}