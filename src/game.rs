use ecs::manager::EntityManager;
use ggez::event::EventHandler;
use ggez::graphics::Color;
use ggez::{Context, GameError, GameResult};

use crate::common::Transform;
use crate::math::Vec2;
use crate::WINDOWS_WIDTH;

#[derive(Default)]
pub struct Game {
    entity_manager: EntityManager,
    setup: bool,
}

#[derive(Debug)]
enum Tag {
    Ui,
}

struct Button {
    title: String,
}

fn render_ui_system(manager: &mut EntityManager, ctx: &mut Context) -> GameResult<()> {
    let buttons = manager.get_entities(Tag::Ui);
    for entity in buttons {
        let transform = Transform::new(Vec2::new(WINDOWS_WIDTH / 2f32, 10f32), Vec2::zero());
        let button = entity.get_component::<Button>().unwrap();
        let text = ggez::graphics::Text::new(button.title.clone());
        ggez::graphics::draw(ctx, &text, transform.position)?;
    }
    Ok(())
}

impl Game {
    fn setup(&mut self) {
        self.setup = true;
        let space_game_btn = self.entity_manager.add_tag(Tag::Ui);
        space_game_btn.add_component(Button {
            title: "Space Game".to_string(),
        });
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
        render_ui_system(&mut self.entity_manager, ctx)?;
        ggez::graphics::present(ctx)
    }
}
