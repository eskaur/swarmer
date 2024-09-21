use crate::common::{Range, Rectangle};
use crate::drawing::draw;
use crate::obstacles::{Obstacle, Tree};
use crate::swarm::{Swarm, Swarmer};
use macroquad::prelude::Vec2;

pub struct GameManager {
    game_board: Rectangle,
    swarm: Swarm,
    obstacles: Vec<Obstacle>,
}

impl GameManager {
    pub fn new(screen_width: f32, screen_height: f32) -> GameManager {
        let mut swarm = Swarm::new();
        for _ in 0..1500 {
            let x = macroquad::rand::gen_range(10.0, screen_width - 10.0);
            let y = macroquad::rand::gen_range(10.0, screen_height - 10.0);

            swarm.add(Swarmer::new(Vec2 { x, y }));
        }
        let obstacles = vec![
            Obstacle::Tree(Tree::new(Vec2 { x: 500.0, y: 500.0 }, 90.0)),
            Obstacle::Tree(Tree::new(
                Vec2 {
                    x: 1000.0,
                    y: 700.0,
                },
                60.0,
            )),
            Obstacle::Tree(Tree::new(
                Vec2 {
                    x: 1200.0,
                    y: 200.0,
                },
                70.0,
            )),
        ];

        GameManager {
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
            obstacles,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.swarm.update(&self.obstacles, &self.game_board, dt);
    }

    pub fn draw(&self) {
        self.draw_swarm();
        self.draw_obstacles();
    }

    fn draw_swarm(&self) {
        self.swarm
            .iter()
            .for_each(|swarmer: &Swarmer| draw(swarmer));
    }

    fn draw_obstacles(&self) {
        self.obstacles
            .iter()
            .for_each(|obstacle: &Obstacle| draw(obstacle));
    }
}
