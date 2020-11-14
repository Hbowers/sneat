use amethyst::{
    core::{timing::Time, SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

pub use crate::components::Velocity;

#[derive(SystemDesc)]
pub struct VelocitySystem;

const X_AIR_DRAG: f32 = 0.95;
const MAX_Y_SPEED: f32 = 1.2;
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
            if velocity.y < MAX_Y_SPEED && velocity.on_floor == false {
                velocity.y = (velocity.y + GRAVITY * time.delta_seconds()).min(MAX_Y_SPEED);
            }

            /* Updating transforms from velocities */
            if velocity.x != 0.0 {
                println!("moving by {}", velocity.x);
                transform.prepend_translation_x(velocity.x);
            };
            if velocity.y != 0.0 {
                transform.prepend_translation_y(velocity.y);
            };

            /* Updating velocities */
            if !velocity.on_floor {
                if velocity.x > 0.0 {
                    velocity.x = (velocity.x - time.delta_seconds() * X_AIR_DRAG).min(0.0);
                }
                if velocity.x < 0.0 {
                    velocity.x = (velocity.x + time.delta_seconds() * X_AIR_DRAG).max(0.0);
                }
            } else {
                velocity.x = 0.0;
                velocity.y = 0.0;
            }
        }
    }
}
