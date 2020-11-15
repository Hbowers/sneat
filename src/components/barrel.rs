use amethyst::{
    ecs::{Component, DenseVecStorage},
};

pub struct  Barrel {
}

impl Barrel {
    pub fn new() -> Barrel {
        Barrel {
        }
    }
}

impl Component for Barrel {
    type Storage = DenseVecStorage<Self>;
}
