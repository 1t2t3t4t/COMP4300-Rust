use ecs::manager::EntityManager;
use ggez::event::EventHandler;
use ggez::graphics::Color;
use ggez::GameError;

#[derive(Default)]
pub struct Game {
    entity_manager: EntityManager,
    setup: bool
}

impl Game {
    fn setup(&mut self) {
        self.setup = true;
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), GameError> {
        if !self.setup {
            self.setup();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        ggez::graphics::clear(ctx, Color::WHITE);

        ggez::graphics::present(ctx)
    }
}
