use std::detect::__is_feature_detected::sha;
use crate::common::Transform;
use crate::space_shooter::component::shape::{Geometry, Shape};
use crate::space_shooter::Tag;
use ecs::entity::Entity;
use ecs::manager::EntityManager;
use ggez::graphics::{Color, DrawMode, Drawable, MeshBuilder, Rect};
use ggez::{Context, GameResult};

fn get_drawable(
    shape: &Shape,
    transform: &Transform,
    ctx: &mut Context,
    draw_mode: DrawMode,
) -> GameResult<impl Drawable> {
    let mut mesh_builder = MeshBuilder::new();
    match shape.geometry {
        Geometry::Triangle => todo!(),
        Geometry::Rectangle => mesh_builder.rectangle(
            draw_mode,
            Rect::new(
                transform.position.x - shape.radius,
                transform.position.y - shape.radius,
                shape.radius * 2f32,
                shape.radius * 2f32,
            ),
            Color::RED,
        ),
        Geometry::Circle => mesh_builder.circle(
            draw_mode,
            transform.position,
            shape.radius,
            0.1,
            Color::RED
        ),
    }?
    .build(ctx)
}

fn render_shapes(entities: &[&mut Entity], ctx: &mut Context) -> GameResult<()> {
    for entity in entities {
        match (
            entity.get_component::<Shape>(),
            entity.get_component::<Transform>(),
        ) {
            (Some(shape), Some(transform)) => {
                let drawable = get_drawable(shape, transform, ctx, DrawMode::fill())?;
                let border = get_drawable(shape, transform, ctx, DrawMode::stroke(3f32))?;
                ggez::graphics::draw(ctx, &drawable, ([0f32, 0f32], Color::BLACK))?;
                ggez::graphics::draw(ctx, &border, ([0f32, 0f32], Color::RED))?;
            }
            _ => debug_assert!(false, "Entity {:?} has invalid component", entity),
        }
    }
    Ok(())
}

pub fn render_shape_system(manager: &mut EntityManager, ctx: &mut Context) -> GameResult<()> {
    render_shapes(&manager.get_entities(Tag::Player), ctx)?;
    render_shapes(&manager.get_entities(Tag::Enemy), ctx)
}
