use macroquad::*;
use legion::*;

mod core;
mod production;
mod resources;
mod people;

use crate::core::*;
use crate::production::*;
// use crate::resources::*;
use crate::people::*;

fn init_colony(world: &mut World) {
    // казарма с рассчетом №1-Ж
    let barracks = install_germ(
        world,
        Tier::T2,
        Area::Military,
    );
    spawn_1_g(world, barracks);

    // T2 производственное помещение под установку верстака, станка, печи, и чанов
    let _manufactory = install_germ(
        world,
        Tier::T2,
        Area::Industrial,
    );

    // T2 Склад с чанами и стеллажами
    let _stock = install_germ (
        world,
        Tier::T2,
        Area::Party,
    );

    // Т1 комнатка для исследований
    let _lab = install_germ (
        world,
        Tier::T1,
        Area::Science,
    );

    // Жилячейки
    let _cells = for _ in 0..35 {
        install_germ(
            world,
            Tier::T1,
            Area::Living,
        );
    };

}

pub struct State {
    world: World
}

impl State {
    pub fn new() -> State {
        let mut world = World::default();
        init_colony(&mut world);
        State {
            world: world
        }
    }

    pub fn turn(&mut self) {
        unimplemented!();
    }

    pub fn stats(&mut self) {
        unimplemented!();
    }
}


#[macroquad::main("Camera")]
async fn main() {
    loop {
        clear_background(RED);

        // Render some primitives in camera space

        set_camera(Camera2D {
            zoom: vec2(1., screen_width() / screen_height()),
            ..Default::default()
        });
        draw_line(-0.4, 0.4, -0.8, 0.9, 0.05, BLUE);
        draw_rectangle(-0.3, 0.3, 0.2, 0.2, GREEN);
        draw_circle(0., 0., 0.1, YELLOW);

        // Back to screen space, render some text

        set_default_camera();
        draw_text("HELLO", 30.0, 200.0, 30.0, BLACK);

        next_frame().await
    }
}
