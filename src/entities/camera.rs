use amethyst::{
    core::{Parent, Transform},
    ecs::Entity,
    prelude::*,
    renderer::camera::Camera,
    window::ScreenDimensions,
};

pub fn initialise_camera(world: &mut World, camera_focus: Entity, width: f32, height: f32) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(Parent {
            entity: camera_focus,
        })
        .with(transform)
        .build();
}
