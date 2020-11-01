use ggez::{Context, ContextBuilder, GameResult, timer};
use ggez::event::{self, EventHandler};
mod state;
mod systems;
mod server;
mod msg;
pub use server::*;
pub use msg::*;

mod client;
pub use client::*;
mod clientdata;
pub use clientdata::*;

pub fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
		.expect("aieee, could not create ggez context!");
    let mut zombie_dodge_ball = Main::new(&mut ctx);
    match event::run(&mut ctx, &mut event_loop, &mut zombie_dodge_ball) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}


struct Main {
    pub tick_rate_ps:u32,
    pub client:Client,
    pub server:Server,
    pub client_results:Vec<ClientData>
}

impl Main {
    pub fn new(ctx: &mut Context) -> Main {
        Main {
            server: Server::new(),
            client: Client::new(ctx),
            tick_rate_ps:20,
            client_results:Vec::new()
        }
    }
}

impl EventHandler for Main {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let delta = 1.0 / self.tick_rate_ps as f32;
        if timer::check_update_time(ctx, self.tick_rate_ps){
            let s = self.server.update(delta, &self.client_results);
            self.client_results.clear();
            self.client.update(s);
        }

		Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let res = self.client.render(ctx, self.tick_rate_ps)?;
        self.client_results.push(res);
        Ok(())
    }
}