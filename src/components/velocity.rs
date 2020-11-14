use amethyst::ecs::{Component, DenseVecStorage};

pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new() -> Velocity{
        Velocity {
            x: 0.0,
            y: 0.0,
        }
    }
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}
