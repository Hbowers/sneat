use amethyst::ecs::{Component, DenseVecStorage};

pub struct  Collidee {}

impl Collidee {
    pub fn new() -> Collidee {
        Collidee {}
    }
}

impl Component for Collidee {
    type Storage = DenseVecStorage<Self>;
}
