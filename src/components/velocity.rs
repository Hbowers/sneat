use amethyst::ecs::{Component, DenseVecStorage};

// TODO: Turn this into *Movement* with a velocity property
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub collided: bool,
}

impl Velocity {
    pub fn new() -> Velocity{
        Velocity {
            x: 0.0,
            y: 0.0,
            collided: false,
        }
    }
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}
