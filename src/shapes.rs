use macroquad::prelude::Vec2;

pub struct Arrow {
    pub position: Vec2,
    pub direction: Vec2,
    pub length: f32,
    pub width: f32,
}

pub struct Circle {
    pub position: Vec2,
    pub diameter: f32,
}

pub enum Shape {
    Arrow(Arrow),
    Circle(Circle),
}
