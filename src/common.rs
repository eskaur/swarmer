pub struct Range {
    pub min: f32,
    pub max: f32,
}

pub struct Rectangle {
    pub xrange: Range,
    pub yrange: Range,
}

pub fn limit(value: f32, limits: &Range) -> f32 {
    if value < limits.min {
        return limits.min;
    } else if value > limits.max {
        return limits.max;
    } else {
        return value;
    }
}
