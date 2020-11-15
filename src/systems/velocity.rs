use amethyst::{
    core::{timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, WriteStorage},
};

pub use crate::components::Velocity;

#[derive(SystemDesc)]
pub struct VelocitySystem;

const X_AIR_DRAG: f32 = 1.15;
const X_DRAG: f32 = 9.;
const MAX_Y_SPEED: f32 = 0.7;
// FIXME: Change back to a meaningful value
const GRAVITY: f32 = -1.;

impl<'s> System<'s> for VelocitySystem {
    type SystemData = (
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut velocities, mut transforms, time): Self::SystemData) {
        for (mut velocity, transform) in (&mut velocities, &mut transforms).join() {
            /* Updating transforms from velocities */
            if velocity.x != 0.0 {
                transform.prepend_translation_x(velocity.x);
            };
            if velocity.y != 0.0 {
                transform.prepend_translation_y(velocity.y);
            };

            if !velocity.collided {
                velocity.y = (velocity.y + GRAVITY * time.delta_seconds())
                    .min(MAX_Y_SPEED)
                    .max(-MAX_Y_SPEED);
            }

            /* Updating velocities */
            if !velocity.collided {
                if velocity.x >= 0.0 {
                    velocity.x = (velocity.x - time.delta_seconds() * X_AIR_DRAG).max(0.);
                }
                if velocity.x <= 0.0 {
                    velocity.x = (velocity.x + time.delta_seconds() * X_AIR_DRAG).min(0.);
                }
            }
            if velocity.collided {
                velocity.y = 0.0;
                if velocity.x >= 0.0 {
                    velocity.x = (velocity.x - time.delta_seconds() * X_DRAG).max(0.);
                }
                if velocity.x <= 0.0 {
                    velocity.x = (velocity.x + time.delta_seconds() * X_DRAG).min(0.);
                }
            }
        }
    }
}
