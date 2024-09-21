mod common;
mod drawing;
mod game;
mod obstacles;
mod shapes;
mod swarm;
mod traits;

use crate::game::GameManager;

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

#[macroquad::main(window_conf)]
async fn main() {
    let screen_width = macroquad::prelude::screen_width();
    let screen_height = macroquad::prelude::screen_height();

    let mut game = GameManager::new(screen_width, screen_height);

    let dt: f32 = 1.0;
    loop {
        // Update game state
        game.update(dt);

        // Draw game state
        macroquad::window::clear_background(macroquad::prelude::DARKGRAY);
        game.draw();

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
