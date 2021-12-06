use ggez::{
    conf::WindowMode, event::EventHandler, graphics::Color, ContextBuilder, GameError, GameResult,
};

mod math;

const WINDOWS_WIDTH: f32 = 1920f32;
const WINDOWS_HEIGHT: f32 = 1080f32;

struct Game;

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        ggez::graphics::clear(ctx, Color::WHITE);

        ggez::graphics::present(ctx)
    }
}

fn main() -> GameResult<()> {
    let (ctx, event_loop) = ContextBuilder::new("Comp4300", "Boss")
        .window_mode(WindowMode::default().dimensions(WINDOWS_WIDTH, WINDOWS_HEIGHT))
        .build()?;

    ggez::event::run(ctx, event_loop, Game)
}
