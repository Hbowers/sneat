use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

pub use crate::components::Velocity;

#[derive(SystemDesc)]
pub struct VelocitySystem;

impl<'s> System<'s> for VelocitySystem {
    type SystemData = (
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (velocities, mut transforms): Self::SystemData) {
        for (velocity, transform) in (&velocities, &mut transforms).join() {
            if velocity.x != 0.0 {
                println!("moving by {}", velocity.x);
                transform.prepend_translation_x(velocity.x);
            };
            if velocity.y != 0.0 {
                transform.prepend_translation_y(velocity.y);
            };
        }
    }
}

