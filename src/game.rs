use ecs::manager::EntityManager;
use ggez::event::EventHandler;
use ggez::graphics::{Color, DrawMode, Font, MeshBuilder, PxScale, Rect, StrokeOptions};
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
    size: f32,
}

fn render_ui_system(manager: &mut EntityManager, ctx: &mut Context) -> GameResult<()> {
    let buttons = manager.get_entities(Tag::Ui);
    let mut y_pos = 10f32;

    for entity in buttons {
        let button = entity.get_component::<Button>().unwrap();

        let mut text = ggez::graphics::Text::new(button.title.clone());
        text.set_font(Font::default(), PxScale::from(button.size));

        let width = (WINDOWS_WIDTH / 2f32) - (text.width(ctx) / 2f32);
        let transform = Transform::new(Vec2::new(width, y_pos), Vec2::zero());
        let position: [f32; 2] = transform.position.into();

        let rect = Rect::new(
            transform.position.x,
            transform.position.y,
            text.width(ctx),
            text.height(ctx),
        );
        let border = MeshBuilder::new()
            .rectangle(DrawMode::Stroke(StrokeOptions::DEFAULT), rect, Color::BLACK)?
            .build(ctx)?;

        ggez::graphics::draw(ctx, &border, ([0f32, 0f32], Color::BLACK))?;
        ggez::graphics::draw(ctx, &text, (position, Color::BLACK))?;

        y_pos += text.height(ctx) + 10f32;
    }
    Ok(())
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
