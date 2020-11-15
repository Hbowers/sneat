use amethyst::{
    assets::Handle,
    core::transform::Transform,
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::components::{Covers, Shape};
use crate::types::Point;
const COVER_WIDTH: f32 = 16.;

pub fn initialise_covering(
    world: &mut World,
    start_x: f32,
    end_x: f32,
    y: f32,
    sprite_sheet_handle: Handle<SpriteSheet>,
) {
    let mut index = start_x;

    while index < end_x {
        initialise_cover_tile(world, (index + COVER_WIDTH / 2., y), sprite_sheet_handle.clone());
        index = index + COVER_WIDTH * 2.;
    }
}

pub fn initialise_cover_tile(
    world: &mut World,
    point: Point,
    sprite_sheet_handle: Handle<SpriteSheet>,
) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
    let mut default_transform = Transform::default();
    default_transform.set_translation_xyz(point.0, point.1, 0.0);

    world
        .create_entity()
        .with(Covers::new(16.0, 16.0))
        .with(Shape::new(16.0, 16.0))
        .with(default_transform.clone())
        .with(sprite_render.clone())
        .build();
}
