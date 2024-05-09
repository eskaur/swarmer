use crate::common::Rectangle;
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
            position,
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
        let speed = self.velocity.length().clamp(MIN_SPEED, MAX_SPEED);
        self.velocity = self.velocity.normalize() * speed;

        // Move according to velocity
        self.position += self.velocity * dt;
        self.position.x = self.position.x.clamp(limits.xrange.min, limits.xrange.max);
        self.position.y = self.position.y.clamp(limits.yrange.min, limits.yrange.max);
    }
}

fn compute_distances(swarmer: &Swarmer, others: &[Swarmer]) -> Vec<f32> {
    others
        .iter()
        .map(|other| swarmer.get_position().distance(other.get_position()))
        .collect()
}

fn filter_by_distance<'a, F>(
    swarmers: &'a [Swarmer],
    distances: &Vec<f32>,
    criterion: F,
) -> Vec<&'a Swarmer>
where
    F: Fn(f32) -> bool,
{
    swarmers
        .iter()
        .zip(distances)
        .filter_map(|(member, &distance)| {
            if criterion(distance) {
                Some(member)
            } else {
                None
            }
        })
        .collect()
}

fn compute_acceleration(current: &Swarmer, everyone: &[Swarmer], limits: &Rectangle) -> Vec2 {
    let distances = compute_distances(current, everyone);

    let is_repulsor = |dist: f32| dist > 0.001 && dist < REPULSION_RANGE;
    let is_attractor = |dist: f32| dist < ATTRACTION_RANGE;

    let repulsors = filter_by_distance(everyone, &distances, is_repulsor);
    let attractors = filter_by_distance(everyone, &distances, is_attractor);

    let repulsion = repulsors
        .iter()
        .fold(Vec2 { x: 0.0, y: 0.0 }, |acc, other| {
            let vec = current.get_position() - other.get_position();
            acc + vec
        });

    let attractors_center_of_mass = attractors
        .iter()
        .fold(Vec2 { x: 0.0, y: 0.0 }, |acc, other| {
            acc + other.get_position()
        })
        / (attractors.len() as f32);

    let attraction = attractors_center_of_mass - current.get_position();

    let attractors_average_velocity = attractors
        .iter()
        .fold(Vec2 { x: 0.0, y: 0.0 }, |acc, other| {
            acc + other.get_velocity()
        })
        / (attractors.len() as f32);
    let alignment = attractors_average_velocity - current.get_velocity();

    let mut wall_repulsion = Vec2 { x: 0.0, y: 0.0 };

    if current.get_position().x - limits.xrange.min < WALL_REPULSION_RANGE {
        let repulsion = 1.0 / (0.01 + current.get_position().x - limits.xrange.min);
        wall_repulsion += Vec2 {
            x: repulsion,
            y: 0.0,
        };
    } else if limits.xrange.max - current.get_position().x < WALL_REPULSION_RANGE {
        let repulsion = -1.0 / (0.01 + limits.xrange.max - current.get_position().x);
        wall_repulsion += Vec2 {
            x: repulsion,
            y: 0.0,
        };
    }

    if current.get_position().y - limits.yrange.min < WALL_REPULSION_RANGE {
        let repulsion = 1.0 / (0.01 + current.get_position().y - limits.yrange.min);
        wall_repulsion += Vec2 {
            x: 0.0,
            y: repulsion,
        };
    } else if limits.yrange.max - current.get_position().y < WALL_REPULSION_RANGE {
        let repulsion = -1.0 / (0.01 + limits.yrange.max - current.get_position().y);
        wall_repulsion += Vec2 {
            x: 0.0,
            y: repulsion,
        };
    }
    REPULSION_FACTOR * repulsion
        + ATTRACTION_FACTOR * attraction
        + ALIGNMENT_FACTOR * alignment
        + WALL_REPULSION_FACTOR * wall_repulsion
}

pub struct Swarm {
    members: Vec<Swarmer>,
}

impl Default for Swarm {
    fn default() -> Self {
        Self::new()
    }
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
        let accelerations: Vec<Vec2> = self
            .members
            .iter()
            .map(|swarmer| compute_acceleration(swarmer, &self.members, limits))
            .collect();

        self.members
            .iter_mut()
            .zip(accelerations)
            .for_each(|(swarmer, acceleration)| swarmer.update(acceleration, limits, dt));
    }
}
