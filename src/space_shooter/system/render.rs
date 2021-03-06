use crate::space_shooter::component::game::Scoreboard;
use crate::space_shooter::component::general::Lifespan;
use crate::space_shooter::component::shape::{Geometry, Shape};
use common::game_transform::GameTransform;
use ecs::entity::Entity;
use ecs::manager::EntityManager;
use ggez::graphics::{Color, DrawMode, Drawable, Font, MeshBuilder, PxScale, Rect, Text};
use ggez::{Context, GameResult};

fn ease_in(progress: f32) -> f32 {
    if progress == 0f32 {
        0f32
    } else {
        2f32.powf(10f32 * progress - 10f32)
    }
}

fn lifespan_color(lifespan: Option<&Lifespan>, mut color: Color) -> Color {
    if let Some(lifespan) = lifespan {
        let progress =
            1f32 - (lifespan.time_left.as_secs_f32() / lifespan.total_time.as_secs_f32());
        color.a = 1f32 - ease_in(progress);
    }

    color
}

fn get_drawable(
    shape: &Shape,
    transform: &GameTransform,
    ctx: &mut Context,
    draw_mode: DrawMode,
    color: Color,
) -> GameResult<impl Drawable> {
    let mut mesh_builder = MeshBuilder::new();
    match shape.geometry {
        Geometry::Rectangle => mesh_builder.rectangle(
            draw_mode,
            Rect::new(
                transform.position.x - shape.radius,
                transform.position.y - shape.radius,
                shape.radius * 2f32,
                shape.radius * 2f32,
            ),
            color,
        ),
        Geometry::Circle => {
            mesh_builder.circle(draw_mode, transform.position, shape.radius, 0.1, color)
        }
    }?
    .build(ctx)
}

fn render_shapes(entities: &[&mut Entity], ctx: &mut Context) -> GameResult<()> {
    for entity in entities {
        if let (Some(shape), Some(transform)) = (
            entity.get_component::<Shape>(),
            entity.get_component::<GameTransform>(),
        ) {
            let lifespan = entity.get_component::<Lifespan>();
            let shape_color = lifespan_color(lifespan, Color::BLACK);
            let border_color = lifespan_color(lifespan, Color::RED);

            let shape_draw = get_drawable(shape, transform, ctx, DrawMode::fill(), shape_color)?;
            let border = get_drawable(shape, transform, ctx, DrawMode::stroke(3f32), border_color)?;

            ggez::graphics::draw(ctx, &shape_draw, ([0f32, 0f32],))?;
            ggez::graphics::draw(ctx, &border, ([0f32, 0f32],))?;
        }
    }
    Ok(())
}

pub fn render_shape_system(manager: &mut EntityManager, ctx: &mut Context) -> GameResult<()> {
    render_shapes(&manager.get_all(), ctx)
}

pub fn render_scoreboard_system(manager: &EntityManager, ctx: &mut Context) -> GameResult<()> {
    let boards = manager.query_entities_component::<Scoreboard>();
    for board in boards {
        let mut text = Text::new(format!("Score: {}", board.current_score));
        text.set_font(Font::default(), PxScale::from(32f32));
        ggez::graphics::draw(ctx, &text, ([12f32, 12f32], Color::BLACK))?;
    }

    Ok(())
}
