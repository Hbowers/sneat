use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::Camera,
};

use crate::components::{Floor, Shape, Sneatling, Velocity, Barrel};
use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::entities::{barrel, floor, sneatling};
use crate::resources::assets;

pub struct Sneat;

impl SimpleState for Sneat {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sneatling_sprite_sheet_handle =
            assets::load_sprite_sheet_by_asset(world, assets::AssetType::Sneatling);
        let environment_sprite_sheet_handle =
            assets::load_sprite_sheet_by_asset(world, assets::AssetType::GroundBlock);
        let barrel_sprite_sheet_handle =
            assets::load_sprite_sheet_by_asset(world, assets::AssetType::Barrel);

        initialise_camera(world);
        world.register::<Sneatling>();
        world.register::<Velocity>();
        world.register::<Barrel>();
        world.register::<Floor>();
        world.register::<Shape>();

        floor::initialise_flooring(world, 2., 90., 16., environment_sprite_sheet_handle);
        barrel::initialise_barrel(world, (23., 32.), barrel_sprite_sheet_handle.clone());
        barrel::initialise_barrel(world, (43., 32.), barrel_sprite_sheet_handle);
        sneatling::initialise_sneatling(world, sneatling_sprite_sheet_handle);
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
