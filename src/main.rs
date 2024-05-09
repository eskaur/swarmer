pub mod common;
pub mod swarm;

use common::{Range, Rectangle};

use macroquad::prelude::Vec2;
use macroquad::{self, window::clear_background};

fn draw_swarmer(swarmer: &swarm::Swarmer) {
    let position = swarmer.get_position();
    let velocity = swarmer.get_velocity();

    let forward = velocity.normalize();
    let sideways = Vec2 {
        x: forward.y,
        y: -forward.x,
    };

    const TRIANGLE_SIZE: f32 = 8.0;

    let v1 = position + TRIANGLE_SIZE * forward;
    let v2 = position - TRIANGLE_SIZE * forward + 0.5 * TRIANGLE_SIZE * sideways;
    let v3 = position - TRIANGLE_SIZE * forward - 0.5 * TRIANGLE_SIZE * sideways;

    macroquad::prelude::draw_triangle(v1, v2, v3, macroquad::prelude::RED);
}

fn draw_swarm(swarm: &swarm::Swarm) {
    swarm
        .iter()
        .for_each(|swarmer: &swarm::Swarmer| draw_swarmer(swarmer));
}

fn window_conf() -> macroquad::window::Conf {
    macroquad::window::Conf {
        window_title: "Swarmer".to_owned(),
        fullscreen: false,
        window_resizable: false,
        window_height: 1080,
        window_width: 1920,
        ..Default::default()
    }
}

struct GameManager {
    game_board: Rectangle,
    swarm: swarm::Swarm,
}

impl GameManager {
    fn update(&mut self, dt: f32) {
        self.swarm.update(&self.game_board, dt);
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let screen_width = macroquad::prelude::screen_width();
    let screen_height = macroquad::prelude::screen_height();

    let mut swarm = swarm::Swarm::new();

    for _ in 0..1500 {
        let x = macroquad::rand::gen_range(10.0, screen_width - 10.0);
        let y = macroquad::rand::gen_range(10.0, screen_height - 10.0);

        swarm.add(swarm::Swarmer::new(Vec2 { x, y }));
    }

    let mut game = GameManager {
        game_board: Rectangle {
            xrange: Range {
                min: 0.0,
                max: screen_width,
            },
            yrange: Range {
                min: 0.0,
                max: screen_height,
            },
        },
        swarm,
    };

    let dt: f32 = 1.0;
    loop {
        // Update game state

        use std::time::Instant;
        let now = Instant::now();

        game.update(dt);

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

        // Draw game state
        clear_background(macroquad::prelude::DARKGRAY);
        draw_swarm(&game.swarm);

        // Misc
        let fps = macroquad::time::get_fps();
        macroquad::prelude::draw_text(
            format!("FPS: {}", fps).as_str(),
            20.0,
            20.0,
            32.0,
            macroquad::prelude::RED,
        );
        macroquad::prelude::next_frame().await
    }
}
