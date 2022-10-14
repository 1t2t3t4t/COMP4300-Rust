use crate::space_shooter::component::game::{CacheDisplayText, DisplayText, DisplayTextEvent};
use crate::space_shooter::Tag;
use crate::{WINDOWS_HEIGHT, WINDOWS_WIDTH};
use common::event::EventReceiver;
use ecs::manager::EntityManager;
use ggez::graphics::{Color, Font, PxScale};
use ggez::{Context, GameResult};
use std::ops::Add;

pub fn lifetime_debug_text_system(
    event_reader: &mut impl EventReceiver<DisplayTextEvent>,
    manager: &mut EntityManager<Tag>,
    ctx: &mut Context,
) {
    let mut display_text = manager.query_entities_tag_mut::<DisplayText>(Tag::Ui);
    if display_text.len() == 0 {
        return;
    }
    let display_text = &mut display_text[0];
    for event in event_reader.read() {
        display_text.texts.push(event);
    }

    let dt = ggez::timer::delta(ctx);
    display_text.texts = display_text
        .texts
        .iter()
        .filter_map(|t| {
            if let Some(new_dur) = t.dur.checked_sub(dt) {
                let mut t = t.clone();
                t.dur = new_dur;
                Some(t)
            } else {
                None
            }
        })
        .collect();
}

pub fn display_debug_text_system(
    manager: &mut EntityManager<Tag>,
    ctx: &mut Context,
) -> GameResult<()> {
    let mut display_text = manager.query_entities_tag_mut::<DisplayText>(Tag::Ui);
    if display_text.len() == 0 {
        return Ok(());
    }
    let display_text = &mut display_text[0];
    let raw_text = display_text.texts.iter().fold(String::new(), |s, t| {
        let new_str = s.add("\n");
        new_str.add(&t.text)
    });
    let should_update_cache = if let Some(cache) = &display_text.cache {
        cache.raw_text != raw_text
    } else {
        true
    };

    if should_update_cache {
        let mut text = ggez::graphics::Text::new(raw_text.clone());
        text.set_font(Font::default(), PxScale::from(15f32));
        let (w, h) = (text.width(ctx), text.height(ctx));
        let position = [WINDOWS_WIDTH / 2f32 - w / 2f32, WINDOWS_HEIGHT - 32f32 - h];
        display_text.cache = Some(CacheDisplayText {
            raw_text,
            text,
            position,
        })
    }
    let cache = display_text.cache.as_ref().unwrap();
    ggez::graphics::draw(ctx, &cache.text, (cache.position, Color::BLACK))
}
