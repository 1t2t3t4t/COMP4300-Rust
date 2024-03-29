use crate::ui::render_fps_system;
use common::event::EventSystem;
use ecs::manager::EntityManager;
use ggez::event::EventHandler;
use ggez::graphics::Color;
use ggez::{Context, GameError};

mod component;
mod system;

pub mod tag {
    pub struct Player;
    pub struct Enemy;
    pub struct Bullet;
    pub struct Ui;
    pub struct Spawner;
}

#[derive(Default)]
pub struct SpaceGame {
    entity_manager: EntityManager,
    event_system: EventSystem,
    setup: bool,
}

impl SpaceGame {
    fn setup(&mut self) {
        self.setup = true;
        component::create_player(&mut self.entity_manager);
        component::create_enemy(&mut self.entity_manager);
        component::create_enemy_spawner(&mut self.entity_manager);
        component::create_bullet_spawner(&mut self.entity_manager);
        component::create_score_board(&mut self.entity_manager);
        component::create_display_text_ui(&mut self.entity_manager);
    }
}

impl EventHandler for SpaceGame {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        if !self.setup {
            self.setup();
        }
        self.entity_manager.update();

        system::ui::lifetime_debug_text_system(
            &mut self.event_system,
            &mut self.entity_manager,
            ctx,
        );

        system::game::lifespan_system(&mut self.entity_manager, ctx)?;
        system::game::enemy_spawner(&mut self.entity_manager, ctx)?;

        system::movement::player_speed_boost_system(
            &mut self.entity_manager,
            ctx,
            &mut self.event_system,
        )?;
        system::movement::player_movement_system(&mut self.entity_manager, ctx)?;
        system::movement::enemy_movement_system(
            &mut self.entity_manager,
            &mut self.event_system,
            ctx,
        )?;
        system::movement::collider_follow_transform_system(&mut self.entity_manager)?;
        system::movement::bullet_movement_system(&mut self.entity_manager, ctx)?;

        system::game::shoot_system(&mut self.entity_manager, ctx)?;
        system::game::kill_enemy_system(&mut self.entity_manager, &mut self.event_system)?;

        system::collision::windows_bound_collision_system(
            &mut self.entity_manager,
            &mut self.event_system,
        )?;
        system::collision::player_collision_system(&mut self.entity_manager)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        ggez::graphics::clear(ctx, Color::WHITE);

        system::render::render_shape_system(&mut self.entity_manager, ctx)?;
        render_fps_system(ctx)?;
        system::game::aim_system(&mut self.entity_manager, ctx)?;
        system::render::render_scoreboard_system(&self.entity_manager, ctx)?;
        system::ui::display_debug_text_system(&mut self.entity_manager, ctx)?;

        ggez::graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }
}
