use ggez::{Context, GameResult, graphics::DrawParam, graphics::{self, Rect}};

use crate::ZombieDodgeBall;

impl ZombieDodgeBall 
{
    pub fn render(&mut self, ctx: &mut Context, alpha:f32) -> GameResult<()>
    {
        let current = &self.current;
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

        for (id, e) in current.entities.iter()
        {
            let mut params = DrawParam::new();
            params.dest.x = e.pos.x;
            params.dest.y = e.pos.y;
            params.scale.x = 1.0 / sprite_size;
            params.scale.y = 1.0 / sprite_size;
            graphics::draw(ctx,   &self.images.spritesheet, params)?;
        }

		graphics::present(ctx)
    }
}
