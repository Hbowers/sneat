use amethyst::{
    core::{timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, WriteStorage},
};

pub use crate::components::Velocity;

#[derive(SystemDesc)]
pub struct VelocitySystem;

const X_DRAG: f32 = 2.;
const MAX_Y_SPEED: f32 = 1.2;
// FIXME: Change back to a meaningful value
const GRAVITY: f32 = 0.;

impl<'s> System<'s> for VelocitySystem {
    type SystemData = (
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut velocities, mut transforms, time): Self::SystemData) {
        for (mut velocity, transform) in (&mut velocities, &mut transforms).join() {
            if velocity.y < MAX_Y_SPEED {
                velocity.y = (velocity.y + GRAVITY * time.delta_seconds()).min(MAX_Y_SPEED);
            }

            /* Updating transforms from velocities */
            if velocity.x != 0.0 {
                transform.prepend_translation_x(velocity.x);
            };
            if velocity.y != 0.0 {
                transform.prepend_translation_y(velocity.y);
            };

            /* Updating velocities */
            if velocity.x > 0.0 {
                velocity.x = (velocity.x - time.delta_seconds() * X_DRAG).min(0.0);
            }
            if velocity.x < 0.0 {
                velocity.x = (velocity.x + time.delta_seconds() * X_DRAG).max(0.0);
            }
        }
    }
}
