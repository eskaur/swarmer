use crate::shapes::{Circle, Shape};
use crate::traits::{Drawable, Object, Repulsive};
use macroquad::prelude::Vec2;

pub struct Tree {
    position: Vec2,
    diameter: f32,
}

impl Tree {
    pub fn new(position: Vec2, diameter: f32) -> Tree {
        Tree { position, diameter }
    }
}

impl Object for Tree {
    fn get_position(&self) -> Vec2 {
        self.position
    }
}

impl Repulsive for Tree {
    fn get_repulsion_vector(&self, object: &impl Object) -> Vec2 {
        let vec = object.get_position() - self.position;
        let dist_from_edge = vec.length() - self.diameter;

        if dist_from_edge <= 0.0 {
            return vec.normalize();
        }
        vec.normalize() / dist_from_edge
    }
}

impl Drawable for Tree {
    fn get_shape(&self) -> Shape {
        Shape::Circle(Circle {
            position: self.position,
            diameter: self.diameter,
        })
    }
}

pub enum Obstacle {
    Tree(Tree),
}

impl Repulsive for Obstacle {
    fn get_repulsion_vector(&self, object: &impl Object) -> Vec2 {
        match self {
            Obstacle::Tree(obj) => obj.get_repulsion_vector(object),
        }
    }
}

impl Drawable for Obstacle {
    fn get_shape(&self) -> Shape {
        match self {
            Obstacle::Tree(obj) => obj.get_shape(),
        }
    }
}
