use ggez::{Context, GameResult};
use ggez::graphics::{Color, Drawable, DrawMode, FillOptions, MeshBuilder, Rect};
use ecs::entity::Entity;
use ecs::manager::EntityManager;
use crate::common::Transform;
use crate::space_shooter::component::shape::{Geometry, Shape};
use crate::space_shooter::Tag;

fn get_drawable(shape: &Shape, transform: &Transform, ctx: &mut Context) -> GameResult<impl Drawable> {
    let mut mesh_builder = MeshBuilder::new();
    match shape.geometry {
        Geometry::Triangle => todo!(),
        Geometry::Rectangle => mesh_builder.rectangle(
            DrawMode::Fill(FillOptions::DEFAULT),
            Rect::new(
                transform.position.x - shape.radius,
                transform.position.y - shape.radius,
                shape.radius * 2f32,
                shape.radius * 2f32
            ),
            Color::RED
        ),
        Geometry::Circle => todo!()
    }?.build(ctx)
}

fn render_shapes(entities: &[&mut Entity], ctx: &mut Context) -> GameResult<()> {
    for entity in entities {
        match (entity.get_component::<Shape>(), entity.get_component::<Transform>()) {
            (Some(shape), Some(transform)) => {
                let drawable = get_drawable(shape, transform, ctx)?;
                ggez::graphics::draw(
                    ctx, &drawable, ([0f32, 0f32], Color::BLACK)
                )?;
            }
            _ => debug_assert!(false, "Entity {:?} has invalid component", entity)
        }
    }
    Ok(())
}

pub fn render_shape_system(manager: &mut EntityManager, ctx: &mut Context) -> GameResult<()> {
    render_shapes(&manager.get_entities(Tag::Player), ctx)?;
    render_shapes(&manager.get_entities(Tag::Enemy), ctx)
}