use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::{prelude::World, Entity},
    core::{
        math::{Vector3},
    },
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::components::{Shape, Sneatling, Velocity, Collider, Coverable, Health, Animation};
use crate::constants::ARENA_HEIGHT;

const SNEATLING_WIDTH: f32 = 6.0;
const SNEATLING_HEIGHT: f32 = 4.0;

pub fn initialise_sneatling(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, sneatling_health: f32) -> Entity {
    let animation = Animation {
        frames: 44,
        frame_duration: 1,
        first_sprite_index: 0,
    };
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);



    let mut default_transform = Transform::default();
    let center = ARENA_HEIGHT / 2.0;
    default_transform.set_translation_xyz(center, center, 0.75);
    default_transform.set_scale(Vector3::new(0.08, 0.08, 1.));

    world
        .create_entity()
        .with(Sneatling::new())
        .with(Collider::new())
        .with(Shape::new(SNEATLING_WIDTH, SNEATLING_HEIGHT))
        .with(Velocity::new())
        .with(Coverable::new())
        .with(Health::new(sneatling_health))
        .with(default_transform)
        .with(sprite_render)
        .with(animation)
        .build()
}
