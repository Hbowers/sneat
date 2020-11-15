use amethyst::ecs::{Component, DenseVecStorage};

pub struct Edible {
    pub in_stomach: bool,
    pub stomach_id: i32,
}

impl Edible {
    pub fn new() -> Edible {
        Edible {
            in_stomach: false,
            stomach_id: -1,
        }
    }
}

impl Component for Edible {
    type Storage = DenseVecStorage<Self>;
}
