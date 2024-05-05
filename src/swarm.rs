use crate::common::{limit, Rectangle};
use macroquad::prelude::Vec2;
use std::slice;

#[derive(Debug)]
pub struct Swarmer {
    position: Vec2,
    velocity: Vec2,
}

const REPULSION_RANGE: f32 = 8.0;
const ATTRACTION_RANGE: f32 = 40.0;
const WALL_REPULSION_RANGE: f32 = 200.0;

const ATTRACTION_FACTOR: f32 = 0.0005;
const REPULSION_FACTOR: f32 = 0.05;
const ALIGNMENT_FACTOR: f32 = 0.05;
const WALL_REPULSION_FACTOR: f32 = 2.0;

const MIN_SPEED: f32 = 3.0;
const MAX_SPEED: f32 = 6.0;

impl Swarmer {
    pub fn new(position: Vec2) -> Swarmer {
        Swarmer {
            position: position,
            velocity: Vec2 { x: 0.1, y: 0.1 },
        }
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn get_velocity(&self) -> Vec2 {
        self.velocity
    }

    fn update(&mut self, acceleration: Vec2, limits: &Rectangle, dt: f32) {
        // Update velocity
        self.velocity += acceleration * dt;

        // Enforce speed limits
        let speed = self.velocity.length();
        if speed > MAX_SPEED {
            self.velocity = self.velocity.normalize() * MAX_SPEED;
        } else if speed < MIN_SPEED {
            self.velocity = self.velocity.normalize() * MIN_SPEED;
        }

        // Move according to velocity
        self.position += self.velocity * dt;
        self.position.x = limit(self.position.x, &limits.xrange);
        self.position.y = limit(self.position.y, &limits.yrange);
    }
}

pub struct Swarm {
    members: Vec<Swarmer>,
}

impl Swarm {
    pub fn new() -> Swarm {
        Swarm {
            members: Vec::new(),
        }
    }

    pub fn add(&mut self, swarmer: Swarmer) {
        self.members.push(swarmer);
    }

    pub fn iter(&self) -> slice::Iter<Swarmer> {
        self.members.iter()
    }

    pub fn update(&mut self, limits: &Rectangle, dt: f32) {
        let mut accelerations: Vec<Vec2> = Vec::new();
        accelerations.reserve(self.members.len());

        for swarmer in self.members.iter() {
            let repulsors: Vec<&Swarmer> = self
                .members
                .iter()
                .filter(|other| {
                    let dist = swarmer.get_position().distance(other.get_position());
                    dist > 0.001 && dist < REPULSION_RANGE
                })
                .collect();

            let attractors: Vec<&Swarmer> = self
                .members
                .iter()
                .filter(|other| {
                    let dist = swarmer.get_position().distance(other.get_position());
                    dist < ATTRACTION_RANGE
                })
                .collect();

            let repulsion = repulsors
                .iter()
                .fold(Vec2 { x: 0.0, y: 0.0 }, |acc, other| {
                    let vec = swarmer.get_position() - other.get_position();
                    acc + vec
                });

            let attractors_center_of_mass = attractors
                .iter()
                .fold(Vec2 { x: 0.0, y: 0.0 }, |acc, other| {
                    acc + other.get_position()
                })
                / (attractors.len() as f32);

            let attraction = attractors_center_of_mass - swarmer.get_position();

            let attractors_average_velocity = attractors
                .iter()
                .fold(Vec2 { x: 0.0, y: 0.0 }, |acc, other| {
                    acc + other.get_velocity()
                })
                / (attractors.len() as f32);
            let alignment = attractors_average_velocity - swarmer.get_velocity();

            let mut wall_repulsion = Vec2 { x: 0.0, y: 0.0 };

            if swarmer.get_position().x - limits.xrange.min < WALL_REPULSION_RANGE {
                let repulsion = 1.0 / (0.01 + swarmer.get_position().x - limits.xrange.min);
                wall_repulsion += Vec2 {
                    x: repulsion,
                    y: 0.0,
                };
            } else if limits.xrange.max - swarmer.get_position().x < WALL_REPULSION_RANGE {
                let repulsion = -1.0 / (0.01 + limits.xrange.max - swarmer.get_position().x);
                wall_repulsion += Vec2 {
                    x: repulsion,
                    y: 0.0,
                };
            }

            if swarmer.get_position().y - limits.yrange.min < WALL_REPULSION_RANGE {
                let repulsion = 1.0 / (0.01 + swarmer.get_position().y - limits.yrange.min);
                wall_repulsion += Vec2 {
                    x: 0.0,
                    y: repulsion,
                };
            } else if limits.yrange.max - swarmer.get_position().y < WALL_REPULSION_RANGE {
                let repulsion = -1.0 / (0.01 + limits.yrange.max - swarmer.get_position().y);
                wall_repulsion += Vec2 {
                    x: 0.0,
                    y: repulsion,
                };
            }

            accelerations.push(
                REPULSION_FACTOR * repulsion
                    + ATTRACTION_FACTOR * attraction
                    + ALIGNMENT_FACTOR * alignment
                    + WALL_REPULSION_FACTOR * wall_repulsion,
            );
        }

        self.members
            .iter_mut()
            .zip(accelerations)
            .for_each(|(swarmer, acceleration)| swarmer.update(acceleration, limits, dt))
    }
}
