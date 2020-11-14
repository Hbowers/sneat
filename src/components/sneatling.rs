use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const SNEATLING_WIDTH: f32 = 68.0;
pub const SNEATLING_HEIGHT: f32 = 46.0;


pub struct Sneatling {
    pub width: f32,
    pub height: f32,
}

impl Sneatling {
    pub fn new() -> Sneatling { 
        Sneatling {
            width: SNEATLING_WIDTH,
            height: SNEATLING_HEIGHT,
        }
    }
}

impl Component for Sneatling {
    type Storage = DenseVecStorage<Self>;
}
