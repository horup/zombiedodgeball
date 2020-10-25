use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
mod state;
use state::State;

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

struct ZombieDodgeBall {
    pub current:State
}

impl ZombieDodgeBall {
    pub fn new(ctx: &mut Context) -> ZombieDodgeBall {
        let bytes = include_bytes!("./resources/spritesheet.png");
        let img = image::load_from_memory(bytes).unwrap();
        
        let img = img.to_rgba();
        let img = graphics::Image::from_rgba8(ctx, img.width() as u16, img.height() as u16, &img).unwrap();


        ZombieDodgeBall {
		    current: State::new()
		}
    }
}

impl EventHandler for ZombieDodgeBall {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
		Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx, graphics::BLACK);
		graphics::present(ctx)
    }
}