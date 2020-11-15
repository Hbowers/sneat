use crate::types::Direction;
use amethyst::ecs::{Component, DenseVecStorage};

pub struct Sneatling {
    pub is_eating: bool,
    pub direction: Direction,
}

impl Sneatling {
    pub fn new() -> Sneatling {
        Sneatling {
            is_eating: false,
            direction: Direction::Left,
        }
    }
}

impl Component for Sneatling {
    type Storage = DenseVecStorage<Self>;
}
