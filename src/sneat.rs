use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const SNEATLING_HEIGHT: f32 = 16.0;
pub const SNEATLING_HEIGHT: f32 = 4.0;

pub struct Sneat;

impl SimpleState for Sneat {
    fn on_start(&mut self, data: StateData<'_, GameData<'_ , '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);
        initialise_camera(world);
        world.register::<Sneat>();

        initialise_sneatling(world, sprite_sheet_handle);
    }
}


