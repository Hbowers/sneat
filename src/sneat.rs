use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::components::{
    Shape,
    Sneatling,
    Velocity,
    Floor,
    Covers,
    Coverable,
};
use crate::resources::assets;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const SNEATLING_WIDTH: f32 = 3.0;
pub const SNEATLING_HEIGHT: f32 = 2.0;

pub struct Sneat;

impl SimpleState for Sneat {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sneatling_sprite_sheet_handle = assets::load_sprite_sheet_by_asset(world, assets::AssetType::Sneatling);
        let environment_sprite_sheet_handle = assets::load_sprite_sheet_by_asset(world,assets::AssetType::GroundBlock);

        initialise_camera(world);
        world.register::<Sneatling>();
        world.register::<Velocity>();
        world.register::<Floor>();
        world.register::<Shape>();
        world.register::<Covers>();
        world.register::<Coverable>();
        initialise_floor(world, environment_sprite_sheet_handle.clone());
        initialise_cover(world, environment_sprite_sheet_handle);
        initialise_sneatling(world, sneatling_sprite_sheet_handle);
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);
    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn initialise_sneatling(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
    let mut default_transform = Transform::default();
    let center = ARENA_HEIGHT / 2.0;
    default_transform.set_translation_xyz(center, center, 0.0);

    world
        .create_entity()
        .with(Sneatling::new())
        .with(Shape::new(SNEATLING_WIDTH, SNEATLING_HEIGHT))
        .with(Velocity::new())
        .with(Coverable::new())
        .with(default_transform)
        .with(sprite_render)
        .build();
}

fn initialise_floor(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
    let mut default_transform = Transform::default();
    let floor_coords = ARENA_HEIGHT / 3.0;
    default_transform.set_translation_xyz(floor_coords, floor_coords, 0.0);

    let mut index = 0;

    while index < 3 {
        world
            .create_entity()
            .with(Floor::new(16.0, 16.0))
            .with(Shape::new(16.0, 16.0))
            .with(default_transform.clone())
            .with(sprite_render.clone())
            .build();
        default_transform.prepend_translation_x(16.0);
        index += 1;
    }
}

fn initialise_cover(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
    let mut default_transform = Transform::default();
    let cover_coords = (ARENA_HEIGHT / 3.0) + 16.0;
    default_transform.set_translation_xyz(cover_coords, cover_coords, -3.);

    world
        .create_entity()
        .with(Covers::new())
        .with(Shape::new(16.0, 16.0))
        .with(default_transform.clone())
        .with(sprite_render.clone())
        .build();
}

