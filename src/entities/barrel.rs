use amethyst::{
    assets::Handle,
    core::transform::Transform,
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::components::{Barrel, Shape, Velocity, Collides};
use crate::types::Point;

pub fn initialise_barrel(
    world: &mut World,
    point: Point,
    sprite_sheet_handle: Handle<SpriteSheet>,
) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
    let mut default_transform = Transform::default();
    default_transform.set_translation_xyz(point.0, point.1, 0.5);

    world
        .create_entity()
        .with(Barrel::new())
        .with(Collides::new())
        .with(Shape::new(4., 5.))
        .with(Velocity::new())
        .with(default_transform.clone())
        .with(sprite_render)
        .build();
}
