use amethyst::ecs::{Component, DenseVecStorage};

// TODO: Turn this into *Movement* with a velocity property
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub collided_x_left: bool,
    pub collided_x_right: bool,
    pub collided_y: bool,
}

impl Velocity {
    pub fn new() -> Velocity {
        Velocity {
            x: 0.0,
            y: 0.0,
            collided_x_left: false,
            collided_x_right: false,
            collided_y: false,
        }
    }
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}
