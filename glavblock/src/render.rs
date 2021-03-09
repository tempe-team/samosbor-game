use legion::*;

use macroquad::prelude::*;
use megaui_macroquad::{
    draw_megaui, draw_window,
    megaui::{hash, Vector2},
};

use crate::people::*;
use crate::storage::*;

pub fn draw_world(
    world: &mut World,
    resources: &mut Resources,
    schedule: &mut Schedule,
) {
    draw_window(
        hash!(),
        Vec2::new(200., 200.),
        Vec2::new(450., 200.),
        None,
        |ui| {
            ui.label(
                None,
                &format!("People"),
            );
            let people = people_by_profession(world);
            for (prof, count) in people.iter() {
                ui.label(
                    None,
                    &format!("{}: {}", *prof, *count),
                );
            };
            ui.label(
                None,
                &format!("Stock"),
            );
            let stock = what_we_have(world);
            for (res, count) in stock.iter() {
                ui.label(
                    None,
                    &format!("{}: {}", *res, (*count).0),
                );
            };
            let mood = block_mood(world);
            ui.label(
                None,
                &format!("Median mood: {}", mood / people.len()),
            );

            let satiety = block_satiety(world);
            ui.label(
                None,
                &format!("Satiety: {}", satiety.0 as usize / people.len()),
            );
            if ui.button(Vector2::new(70., 70.), "Turn") {
                schedule.execute(
                    world,
                    resources,
                );
            }
        },
    );
    draw_megaui();
}
