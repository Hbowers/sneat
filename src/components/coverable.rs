use amethyst::{
    ecs::{Component, DenseVecStorage},
};

const MIN_COVERED_AMOUNT: f32 = 80.0;

pub struct Coverable {
    pub covered_amount: f32,
    pub time_out_of_cover: f32
}

impl Coverable {
    pub fn new() -> Coverable {
        Coverable {
            covered_amount: 0.0,
            time_out_of_cover: 0.0,
        }
    }

    pub fn in_cover(&self) -> bool {
        self.covered_amount >= MIN_COVERED_AMOUNT
    }
}

impl Component for Coverable {
    type Storage = DenseVecStorage<Self>;
}
