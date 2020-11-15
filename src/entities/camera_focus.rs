use amethyst::{
    core::Transform,
    ecs::Entity,
    prelude::*,
    renderer::transparent::Transparent,
};

use crate::components::CameraFocus;

pub fn initialise_camera_focus(world: &mut World, x: f32, y: f32, z: f32) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(x,y,z);

    world
        .create_entity()
        .with(transform)
        .with(CameraFocus::new())
        .with(Transparent)
        .build()
}


