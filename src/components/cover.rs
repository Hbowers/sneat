use amethyst::{
    ecs::{Component, DenseVecStorage},
};

pub struct Cover {
    pub quality: f32
}

impl Cover {
    pub fn new() -> Cover {
        Cover {
            quality: 100.0
        }
    }
}

impl Component for Cover {
    type Storage = DenseVecStorage<Self>;
}
