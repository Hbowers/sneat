use amethyst::ecs::{Component, DenseVecStorage};

pub struct Covers {
    pub quality: f32,
    pub width: f32,
    pub height: f32,
}

impl Covers {
    pub fn new(width: f32, height: f32) -> Covers {
        Covers {
            quality: 100.0,
            width,
            height,
        }
    }
}

impl Component for Covers {
    type Storage = DenseVecStorage<Self>;
}
