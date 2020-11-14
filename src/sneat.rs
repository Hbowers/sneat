use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};


use crate::components::Sneatling;
use crate::components::Floor;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub struct Sneat;

impl SimpleState for Sneat {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sneatling_sprite_sheet_handle = load_sprite_sheet(world, "sprites/Whale/1-Idle/1.png", "sprite_configs/sneatling_spritesheet.ron");
        let environment_sprite_sheet_handle = load_sprite_sheet(world, "sprites/environment/tileset.png", "sprite_configs/environment_spritesheet.ron");
        initialise_camera(world);
        world.register::<Sneatling>();
        world.register::<Floor>();
        initialise_sneatling(world, sneatling_sprite_sheet_handle);
        initialise_floor(world, environment_sprite_sheet_handle);
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
            .with(default_transform.clone())
            .with(sprite_render.clone())
            .build();
        default_transform.prepend_translation_x(16.0);
        index += 1;
    }
}

fn load_sprite_sheet(world: &mut World, path_to_sprite: &str, path_to_sprite_config: &str) -> Handle<SpriteSheet> {
    let texture_handler = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            path_to_sprite,
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        path_to_sprite_config,
        SpriteSheetFormat(texture_handler),
        (),
        &sprite_sheet_store,
    )
}

