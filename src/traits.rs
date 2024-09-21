use crate::shapes::Shape;
use macroquad::prelude::Vec2;

pub trait Object {
    fn get_position(&self) -> Vec2;
}

pub trait Movable {
    fn get_velocity(&self) -> Vec2;
}

pub trait Repulsive {
    fn get_repulsion_vector(&self, object: &impl Object) -> Vec2;
}

pub trait Drawable {
    fn get_shape(&self) -> Shape;
}
