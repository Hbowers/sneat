use amethyst::{
    ecs::{Component, DenseVecStorage},
};

pub struct  Health {
    pub value: f32
}

impl Health {
    pub fn new(value: f32) -> Health {
        Health {
            value
        }
    }
}

impl Component for Health {
    type Storage = DenseVecStorage<Self>;
}