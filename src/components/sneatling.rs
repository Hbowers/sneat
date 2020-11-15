use amethyst::ecs::{Component, DenseVecStorage};

pub struct  Sneatling {
    pub is_eating: bool,
}

impl Sneatling {
    pub fn new() -> Sneatling {
        Sneatling { is_eating: false }
    }
}

impl Component for Sneatling {
    type Storage = DenseVecStorage<Self>;
}
