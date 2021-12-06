use ecs::manager::EntityManager;
use ggez::event::EventHandler;
use ggez::graphics::Color;
use ggez::GameError;

use crate::ui::{render_ui_system, Button};

#[derive(Default)]
pub struct Game {
    entity_manager: EntityManager,
    setup: bool,
}

#[derive(Debug)]
pub enum Tag {
    Ui,
}

impl Game {
    fn setup(&mut self) {
        self.setup = true;
        let space_game_btn = self.entity_manager.add_tag(Tag::Ui);
        space_game_btn.add_component(Button {
            title: "Space Game".to_string(),
            size: 50f32,
        });
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), GameError> {
        if !self.setup {
            self.setup();
        }
        self.entity_manager.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        ggez::graphics::clear(ctx, Color::WHITE);

        render_ui_system(&mut self.entity_manager, ctx)?;

        ggez::graphics::present(ctx)
    }
}
