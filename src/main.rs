use game::Game;
use ggez::{conf::WindowMode, ContextBuilder, GameResult};

mod common;
mod game;
mod math;
mod space_shooter;
mod ui;

const WINDOWS_WIDTH: f32 = 1920f32;
const WINDOWS_HEIGHT: f32 = 1080f32;

fn main() -> GameResult<()> {
    let (ctx, event_loop) = ContextBuilder::new("Comp4300", "Boss")
        .window_mode(WindowMode::default().dimensions(WINDOWS_WIDTH, WINDOWS_HEIGHT))
        .build()?;

    ggez::event::run(ctx, event_loop, Game::default())
}
