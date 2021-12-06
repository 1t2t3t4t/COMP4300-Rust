use ecs::manager::EntityManager;
use ggez::{
    graphics::{Color, DrawMode, Font, MeshBuilder, PxScale, Rect, StrokeOptions},
    Context, GameResult,
};

use crate::{
    common::{Transform, TryGet},
    game::Tag,
    math::Vec2,
    WINDOWS_WIDTH,
};

pub struct Button {
    pub title: String,
    pub size: f32,
}

pub fn render_ui_system(manager: &mut EntityManager, ctx: &mut Context) -> GameResult<()> {
    let buttons = manager.get_entities(Tag::Ui);
    let mut y_pos = 10f32;

    for entity in buttons {
        let button = entity.try_get_component::<Button>()?;

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
