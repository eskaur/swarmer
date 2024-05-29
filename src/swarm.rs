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
const REPULSION_FACTOR: f32 = 0.2;
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

fn compute_repulsion(swarmer: &Swarmer, others: &[&Swarmer]) -> Vec2 {
    others.iter().fold(Vec2 { x: 0.0, y: 0.0 }, |acc, other| {
        let vec = swarmer.get_position() - other.get_position();
        acc + vec.normalize()
    })
}

fn compute_attraction(swarmer: &Swarmer, others: &[&Swarmer]) -> Vec2 {
    let attractors_center_of_mass = others.iter().fold(Vec2 { x: 0.0, y: 0.0 }, |acc, other| {
        acc + other.get_position()
    }) / (others.len() as f32);
    attractors_center_of_mass - swarmer.get_position()
}

fn compute_alignment(swarmer: &Swarmer, others: &[&Swarmer]) -> Vec2 {
    let attractors_average_velocity = others.iter().fold(Vec2 { x: 0.0, y: 0.0 }, |acc, other| {
        acc + other.get_velocity()
    }) / (others.len() as f32);
    attractors_average_velocity - swarmer.get_velocity()
}

fn compute_wall_repulsion(swarmer: &Swarmer, walls: &Rectangle) -> Vec2 {
    let mut wall_repulsion = Vec2 { x: 0.0, y: 0.0 };

    if swarmer.get_position().x - walls.xrange.min < WALL_REPULSION_RANGE {
        let repulsion = 1.0 / (0.01 + swarmer.get_position().x - walls.xrange.min);
        wall_repulsion += Vec2 {
            x: repulsion,
            y: 0.0,
        };
    } else if walls.xrange.max - swarmer.get_position().x < WALL_REPULSION_RANGE {
        let repulsion = -1.0 / (0.01 + walls.xrange.max - swarmer.get_position().x);
        wall_repulsion += Vec2 {
            x: repulsion,
            y: 0.0,
        };
    }

    if swarmer.get_position().y - walls.yrange.min < WALL_REPULSION_RANGE {
        let repulsion = 1.0 / (0.01 + swarmer.get_position().y - walls.yrange.min);
        wall_repulsion += Vec2 {
            x: 0.0,
            y: repulsion,
        };
    } else if walls.yrange.max - swarmer.get_position().y < WALL_REPULSION_RANGE {
        let repulsion = -1.0 / (0.01 + walls.yrange.max - swarmer.get_position().y);
        wall_repulsion += Vec2 {
            x: 0.0,
            y: repulsion,
        };
    }
    wall_repulsion
}

fn compute_acceleration(current: &Swarmer, everyone: &[Swarmer], limits: &Rectangle) -> Vec2 {
    let distances = compute_distances(current, everyone);

    let is_repulsor = |dist: f32| dist > 0.001 && dist < REPULSION_RANGE;
    let is_attractor = |dist: f32| dist < ATTRACTION_RANGE;

    let repulsors = filter_by_distance(everyone, &distances, is_repulsor);
    let attractors = filter_by_distance(everyone, &distances, is_attractor);

    let repulsion = compute_repulsion(current, &repulsors);
    let attraction = compute_attraction(current, &attractors);
    let alignment = compute_alignment(current, &attractors);
    let wall_repulsion = compute_wall_repulsion(current, limits);

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
