use amethyst::ecs::{Component, DenseVecStorage};

use crate::types::Point;

pub struct SpitTravel {
    pub position: Point,
    pub velocity: Point,
}

impl SpitTravel {
    pub fn new(position: Point, velocity: Point) -> SpitTravel {
        SpitTravel { position, velocity }
    }
}

impl Component for SpitTravel {
    type Storage = DenseVecStorage<Self>;
}
