use amethyst::ecs::{Component, DenseVecStorage};

pub struct Shape {
    pub width: f32,
    pub height: f32,
}

impl Shape {
    pub fn new(width: f32, height: f32) -> Shape{
        Shape {
            width,
            height,
        }
    }
}

impl Component for Shape {
    type Storage = DenseVecStorage<Self>;
}
