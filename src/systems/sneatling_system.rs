use amethyst::{
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

pub use crate::components::Sneatling;
pub use crate::components::Velocity;

#[derive(SystemDesc)]
pub struct SneatlingMovementSystem;

const SNEATLING_SPEED: f32 = 1.2;

impl<'s> System<'s> for SneatlingMovementSystem {
    type SystemData = (
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Sneatling>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut velocities, sneatlings, input): Self::SystemData) {
        for (_sneatling, velocity) in (&sneatlings, &mut velocities).join() {
            let movement = input.axis_value("player_1_walk");
            let jump = input.action_is_down("player_1_jump").unwrap_or(false);
            if jump {
                velocity.y = 3.;
            }
            if velocity.y != 0.0 {
                return;
            }
            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let scaled_movement = SNEATLING_SPEED * mv_amount;
                    let new_velocity = velocity.x + scaled_movement;
                    velocity.x =
                        match new_velocity.abs() > (SNEATLING_SPEED * scaled_movement).abs() {
                            true => scaled_movement,
                            false => new_velocity,
                        };
                }
            }
        }
    }
}
