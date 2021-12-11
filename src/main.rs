use std::mem::MaybeUninit;
use crate::space_shooter::SpaceGame;
use ggez::{conf::WindowMode, ContextBuilder, GameResult};
use pprof::ProfilerGuard;

mod common;
mod game;
mod math;
mod space_shooter;
mod ui;

const WINDOWS_WIDTH: f32 = 1280f32;
const WINDOWS_HEIGHT: f32 = 720f32;

static mut PROFILER: MaybeUninit<ProfilerGuard> = MaybeUninit::uninit();
const ONCE: std::sync::Once = std::sync::Once::new();

pub fn profiler() -> &ProfilerGuard {
    ONCE.call_once(|| {
        unsafe {
            PROFILER.write(ProfilerGuard::new(100).unwrap().blocklist(&["libc", "libgcc", "pthread"]));
        }
    });
    unsafe {
        PROFILER.assume_init_ref()
    }
}

fn main() -> GameResult<()> {
    profiler();
    let (ctx, event_loop) = ContextBuilder::new("Comp4300", "Boss")
        .window_mode(WindowMode::default().dimensions(WINDOWS_WIDTH, WINDOWS_HEIGHT))
        .build()?;

    ggez::event::run(ctx, event_loop, SpaceGame::default())
}
