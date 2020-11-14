use amethyst::{
    ecs::{Component, DenseVecStorage},
};

pub struct Floor {
    pub width: f32,
    pub height: f32,
}

impl Floor {
    pub fn new(width: f32, height: f32) -> Floor {
        Floor {
            width,
            height
        }
    }
}

impl Component for Floor {
    type Storage = DenseVecStorage<Self>;
}