use amethyst::ecs::{Component, DenseVecStorage};

pub struct Sneatling {}

impl Sneatling {
    pub fn new() -> Sneatling {
        Sneatling {}
    }
}

impl Component for Sneatling {
    type Storage = DenseVecStorage<Self>;
}
