use amethyst::{
    assets::Handle,
    core::transform::Transform,
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::components::{Floor, Shape};
use crate::types::Point;
const FLOOR_WIDTH: f32 = 8.;

pub fn initialise_flooring(
    world: &mut World,
    start_x: f32,
    end_x: f32,
    y: f32,
    sprite_sheet_handle: Handle<SpriteSheet>,
) {
    let mut index = start_x;

    while index < end_x {
        initialise_floor_tile(world, (index + FLOOR_WIDTH / 2., y), sprite_sheet_handle.clone());
        index += FLOOR_WIDTH;
    }
}

pub fn initialise_floor_tile(
    world: &mut World,
    point: Point,
    sprite_sheet_handle: Handle<SpriteSheet>,
) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
    let mut default_transform = Transform::default();
    default_transform.set_translation_xyz(point.0, point.1, 0.0);

    world
        .create_entity()
        .with(Floor::new(FLOOR_WIDTH, 8.0))
        .with(Shape::new(FLOOR_WIDTH, 8.0))
        .with(default_transform)
        .with(sprite_render)
        .build();
}
