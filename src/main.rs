use ggez::{ContextBuilder, event::EventHandler, GameError, GameResult, conf::WindowMode};

mod math;

const WINDOWS_WIDTH: f32 = 1920f32;
const WINDOWS_HEIGHT: f32 = 1080f32;

struct Game;

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> Result<(), GameError> {
        Ok(())
    }
}

fn main() -> GameResult<()> {
    let (ctx, event_loop) = ContextBuilder::new("Comp4300", "Boss")
        .window_mode(WindowMode::default().dimensions(WINDOWS_WIDTH, WINDOWS_HEIGHT))
        .build()?;
    
    ggez::event::run(ctx, event_loop, Game)
}
