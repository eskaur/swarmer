use crate::shapes::{Arrow, Circle, Shape};
use crate::traits::Drawable;
use macroquad::prelude::Vec2;

fn draw_arrow(arrow: &Arrow) {
    let position = arrow.position;
    let forward = arrow.direction;
    let sideways = Vec2 {
        x: forward.y,
        y: -forward.x,
    };
    let v1 = position + arrow.length * forward;
    let v2 = position - arrow.length * forward + arrow.width * sideways;
    let v3 = position - arrow.length * forward - arrow.width * sideways;
    macroquad::prelude::draw_triangle(v1, v2, v3, macroquad::prelude::RED);
}

fn draw_circle(circle: &Circle) {
    macroquad::prelude::draw_circle(
        circle.position.x,
        circle.position.y,
        0.5 * circle.diameter,
        macroquad::prelude::DARKGREEN,
    )
}

pub fn draw(object: &impl Drawable) {
    let shape = object.get_shape();
    match shape {
        Shape::Circle(circle) => draw_circle(&circle),
        Shape::Arrow(arrow) => draw_arrow(&arrow),
    }
}
