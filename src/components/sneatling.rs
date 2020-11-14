use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};


pub struct Sneatling {}

impl Sneatling {
    pub fn new() -> Sneatling { 
        Sneatling {}
    }
}

impl Component for Sneatling {
    type Storage = DenseVecStorage<Self>;
}
