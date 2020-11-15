use amethyst::ecs::{Component, DenseVecStorage};

pub struct  Collides {}

impl Collides {
    pub fn new() -> Collides {
        Collides {}
    }
}

impl Component for Collides {
    type Storage = DenseVecStorage<Self>;
}
