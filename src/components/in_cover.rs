use amethyst::{
    ecs::{Component, DenseVecStorage},
};

pub struct InCover {
    pub quality: f32
}

impl InCover {
    pub fn new() -> Cover {
        Cover {}
    }
}

impl Component for InCover {
    type Storage = DenseVecStorage<Self>;
}
