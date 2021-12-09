use crate::ui::render_fps_system;
use ecs::manager::EntityManager;
use ggez::event::EventHandler;
use ggez::graphics::Color;
use ggez::{Context, GameError};

mod component;
mod system;

#[derive(Debug)]
enum Tag {
    Player,
    Enemy,
    Bullet,
    Ui,
    Spawner,
}

#[derive(Default)]
pub struct SpaceGame {
    entity_manager: EntityManager,
    setup: bool,
}

impl SpaceGame {
    fn setup(&mut self) {
        self.setup = true;
        component::create_player(&mut self.entity_manager);
        component::create_enemy(&mut self.entity_manager);
        component::create_enemy_spawner(&mut self.entity_manager);
    }
}

impl EventHandler for SpaceGame {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        if !self.setup {
            self.setup();
        }
        self.entity_manager.update();

        system::game::enemy_spawner(&mut self.entity_manager, ctx)?;
        system::movement::player_movement_system(&mut self.entity_manager, ctx)?;
        system::movement::enemy_movement_system(&mut self.entity_manager, ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        ggez::graphics::clear(ctx, Color::WHITE);
        system::render::render_shape_system(&mut self.entity_manager, ctx)?;
        render_fps_system(ctx)?;
        ggez::graphics::present(ctx)
    }
}
