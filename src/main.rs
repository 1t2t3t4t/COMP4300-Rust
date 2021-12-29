use crate::space_shooter::SpaceGame;
use ggez::{conf::WindowMode, ContextBuilder, GameResult};

mod game;
mod space_shooter;
mod ui;

const WINDOWS_WIDTH: f32 = 1280f32;
const WINDOWS_HEIGHT: f32 = 720f32;

fn main() -> GameResult<()> {
    let (ctx, event_loop) = ContextBuilder::new("Comp4300", "Boss")
        .window_mode(WindowMode::default().dimensions(WINDOWS_WIDTH, WINDOWS_HEIGHT))
        .build()?;

    ggez::event::run(ctx, event_loop, SpaceGame::default())
}
