use amethyst::{
    ecs::{Component, DenseVecStorage},
};

pub struct Coverable {
    pub covered_amount: f32
}

impl Coverable {
    pub fn new() -> Coverable {
        Coverable {
            covered_amount: 0.0
        }
    }
}

impl Component for Coverable {
    type Storage = DenseVecStorage<Self>;
}
