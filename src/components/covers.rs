use amethyst::{
    ecs::{Component, DenseVecStorage},
};

pub struct Covers {
    pub quality: f32
}

impl Covers {
    pub fn new() -> Covers {
        Covers {
            quality: 100.0
        }
    }
}

impl Component for Covers {
    type Storage = DenseVecStorage<Self>;
}
