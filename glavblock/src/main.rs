use macroquad::*;
use legion::*;

mod core;
mod production;
mod resources;
mod storage;
mod people;
mod area;

use crate::core::*;
use crate::production::*;
use crate::resources::*;
use crate::storage::*;
use crate::people::*;
use crate::area::*;

fn init_colony(world: &mut World) {
    // казарма с рассчетом №1-Ж
    let barracks = install_germ(
        world,
        Tier::T2,
        AreaType::Military,
    );
    spawn_1_g(world, barracks);

    // T2 производственное помещение под установку верстака, станка, печи, и чанов
    let _manufactory = install_germ(
        world,
        Tier::T2,
        AreaType::Industrial,
    );

    // T2 Склад с чанами и стеллажами
    let stock = install_germ(
        world,
        Tier::T2,
        AreaType::Party,
    );

    // Т1 комнатка для исследований
    install_germ (
        world,
        Tier::T1,
        AreaType::Science,
    );

    let start_sci_spec = random_sci_spec();
    let cell_sciencists = install_germ(
        world,
        Tier::T1,
        AreaType::Living,
    );
    spawn_comrad(
        world,
        Profession::Scientist,
        Tier::T1,
        ArmsSlot::ScienceToolSetT1,
        FaceSlot::Respirator,
        HeadSlot::Empty,
        TorsoSlot::Robe,
        None,
        Some(start_sci_spec),
        cell_sciencists,
    );

    // Жилячейки
    for _ in 0..33 {
        let cell = install_germ(
            world,
            Tier::T1,
            AreaType::Living,
        );
        for _ in 0..3 {
            spawn_comrad(
                world,
                Profession::Worker,
                Tier::T1,
                ArmsSlot::WorkToolSetT1,
                FaceSlot::Respirator,
                HeadSlot::Empty,
                TorsoSlot::Robe,
                None,
                None,
                cell_sciencists,
            );
        }
    };

    // Ресурсы

    put_resource(
        world,
        Resource::Concentrat,
        Some(Tier::T2),
        RealUnits(100),
    );

    put_resource(
        world,
        Resource::Concentrat,
        Some(Tier::T1),
        RealUnits(1000),
    );
    put_resource(
        world,
        Resource::Scrap,
        Some(Tier::T1),
        RealUnits(500),
    );
    put_resource(
        world,
        Resource::Scrap,
        Some(Tier::T2),
        RealUnits(50),
    );

    put_resource(
        world,
        Resource::Polymer,
        Some(Tier::T1),
        RealUnits(100),
    );
    put_resource(
        world,
        Resource::Polymer,
        Some(Tier::T2),
        RealUnits(10),
    );
}

fn turn(world: &mut World) {
    unimplemented!();
}

#[macroquad::main("Главблок")]
async fn main() {
    let mut world = World::default();
    init_colony(&mut world);
    loop {
        clear_background(WHITE);

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

        turn(&mut world);
        next_frame().await
    }
}
