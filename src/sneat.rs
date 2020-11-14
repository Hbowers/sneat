use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::components::{Shape, Sneatling, Velocity};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const SNEATLING_WIDTH: f32 = 68.0;
pub const SNEATLING_HEIGHT: f32 = 46.0;

pub struct Sneat;

impl SimpleState for Sneat {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);
        initialise_camera(world);
        world.register::<Sneatling>();
        world.register::<Velocity>();
        world.register::<Shape>();
        initialise_sneatling(world, sprite_sheet_handle);
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
        .with(default_transform)
        .with(sprite_render)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handler = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/Whale/1-Idle/1.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        "sprite_configs/sneatling_spritesheet.ron",
        SpriteSheetFormat(texture_handler),
        (),
        &sprite_sheet_store,
    )
}
