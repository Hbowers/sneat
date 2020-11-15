use amethyst::{
    assets::Handle,
    core::transform::Transform,
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};


use crate::components::{Covers, Shape};
use crate::types::Point;
const COVER_WIDTH: f32 = 8.;
const COVER_LAYER: f32 = 0.;

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
        index += COVER_WIDTH;
    }
}

pub fn initialise_cover_tile(
    world: &mut World,
    point: Point,
    sprite_sheet_handle: Handle<SpriteSheet>,
) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);
    let mut default_transform = Transform::default();
    default_transform.set_translation_xyz(point.0, point.1, COVER_LAYER);

    world
        .create_entity()
        .with(Covers::new(COVER_WIDTH, 8.0))
        .with(Shape::new(COVER_WIDTH, 8.0))
        .with(default_transform)
        .with(sprite_render)
        .build();
}
