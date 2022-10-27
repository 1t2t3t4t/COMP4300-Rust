use common::game_transform::{GameTransform, TryGet};
use common::math::Vec2;
use ecs::manager::EntityManager;
use ggez::{
    graphics::{Color, DrawMode, Font, MeshBuilder, PxScale, Rect, StrokeOptions},
    Context, GameResult,
};

use crate::space_shooter::tag;
use crate::{WINDOWS_HEIGHT, WINDOWS_WIDTH};

pub struct Button {
    pub title: String,
    pub size: f32,
}

pub fn render_ui_system(manager: &mut EntityManager, ctx: &mut Context) -> GameResult<()> {
    let buttons = manager.get_entities_with_tag_mut::<tag::Ui>();
    let mut y_pos = 10f32;

    for entity in buttons {
        let button = entity.try_get_component::<Button>()?;

        let mut text = ggez::graphics::Text::new(button.title.clone());
        text.set_font(Font::default(), PxScale::from(button.size));

        let width = (WINDOWS_WIDTH / 2f32) - (text.width(ctx) / 2f32);
        let transform = GameTransform::new(Vec2::new(width, y_pos), Vec2::zero());
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

pub fn render_fps_system(ctx: &mut Context) -> GameResult<()> {
    let dt = ggez::timer::delta(ctx);
    let fps = ggez::timer::fps(ctx);

    let mut text =
        ggez::graphics::Text::new(format!("fps: {}; delta: {}", fps.round(), dt.as_millis()));
    text.set_font(Font::default(), PxScale::from(15f32));
    let (w, h) = (text.width(ctx), text.height(ctx));
    let position = [WINDOWS_WIDTH - 10f32 - w, WINDOWS_HEIGHT - 10f32 - h];
    ggez::graphics::draw(ctx, &text, (position, Color::BLACK))
}
