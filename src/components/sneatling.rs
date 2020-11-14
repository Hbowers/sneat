use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const SNEATLING_HEIGHT: f32 = 16.0;
pub const SNEATLING_HEIGHT: f32 = 4.0;


pub struct Sneatling {
    pub width: f32,
    pub height: f32,
}

impl Sneatling {
    fn new() -> Sneatling { 
        Sneatling {
            width: SNEATLING_WIDTH,
            hegiht: SNEATLING_HEIGHT,
        }
    }
}

impl Component for Sneatling {
    type Storage = DenseVecStorage<Self>;
}
