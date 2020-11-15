use amethyst::ecs::{Component, DenseVecStorage};

pub struct  Edible {
    pub in_stomach: bool,
}

impl Edible {
    pub fn new() -> Edible {
        Edible { in_stomach: false }
    }
}

impl Component for Edible {
    type Storage = DenseVecStorage<Self>;
}
