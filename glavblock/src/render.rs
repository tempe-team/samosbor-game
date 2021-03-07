use legion::*;

use macroquad::prelude::*;
use megaui_macroquad::{
    draw_megaui, draw_window,
    megaui::{hash, Vector2},
};

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
                &format!("Концетрат: {}", 20),
            );

            if ui.button(Vector2::new(10., 55.), "Ход") {
                schedule.execute(
                    world,
                    resources,
                );
            }
        },
    );
    draw_megaui();
}
