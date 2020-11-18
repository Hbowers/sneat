pub fn clamp(min: f32, max: f32, mut value: f32) -> f32 {
    assert!(min <= max);
    if value < min {
        value = min;
    }
    if value > max {
        value = max;
    }
    value
}

pub fn point_within_range(min: f32, max: f32, point: f32) -> bool {
    if point < min || point > max {
        false
    } else {
        true
    }
}
