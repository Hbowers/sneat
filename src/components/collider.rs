use amethyst::ecs::{Component, DenseVecStorage};

pub struct Collider {}

impl Collider {
    pub fn new() -> Collider {
        Collider {}
    }
}

impl Component for Collider {
    type Storage = DenseVecStorage<Self>;
}
