use amethyst::{
    core::{SystemDesc, Transform, timing::Time},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

pub use crate::components::Velocity;

#[derive(SystemDesc)]
pub struct VelocitySystem;

const X_DRAG: f32 = 2.;

impl<'s> System<'s> for VelocitySystem {
    type SystemData = (
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut velocities, mut transforms, time): Self::SystemData) {
        for (mut velocity, transform) in (&mut velocities, &mut transforms).join() {
            if velocity.x != 0.0 {
                println!("moving by {}", velocity.x);
                transform.prepend_translation_x(velocity.x);
            };
            if velocity.y != 0.0 {
                transform.prepend_translation_y(velocity.y);
            };
            if velocity.x > 0.0 {
                velocity.x = (velocity.x - time.delta_seconds() * X_DRAG).min(0.0);
            }
            if velocity.x < 0.0 {
                velocity.x = (velocity.x + time.delta_seconds() * X_DRAG).max(0.0);
            }
        }
    }
}

