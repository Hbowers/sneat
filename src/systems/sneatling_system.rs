use amethyst::{
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

pub use crate::components::Sneatling;
pub use crate::components::Velocity;

#[derive(SystemDesc)]
pub struct SneatlingMovementSystem;

const SNEATLING_SPEED: f32 = 0.3;
const SNEATLING_AIR_SPEED: f32 = 0.2;
const SNEATLING_JUMP_HEIGHT: f32 = 0.6;

impl<'s> System<'s> for SneatlingMovementSystem {
    type SystemData = (
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Sneatling>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut velocities, sneatlings, input): Self::SystemData) {
        for (_sneatling, velocity) in (&sneatlings, &mut velocities).join() {
            let movement = input.axis_value("player_1_walk");

            /* Actions where the Sneatling can be falling */
            let eat = input.action_is_down("player_1_eat").unwrap_or(false);
            if eat {
                println!("EAT!")
            };
            if !velocity.on_floor {
                if let Some(mv_amount) = movement {
                    if mv_amount != 0.0 {
                        let scaled_movement = SNEATLING_AIR_SPEED * mv_amount;
                        let new_velocity = velocity.x + scaled_movement;
                        if new_velocity > 0.0 && velocity.x < SNEATLING_AIR_SPEED {
                            velocity.x = new_velocity.min(SNEATLING_SPEED);
                        }
                        if new_velocity < 0.0 && velocity.x > -SNEATLING_AIR_SPEED {
                            velocity.x = new_velocity.max(-SNEATLING_SPEED);
                        }
                    }
                }
            }

            if velocity.on_floor {
                /* Actions Where the Sneatling is on the floor*/
                let jump = input.action_is_down("player_1_jump").unwrap_or(false);
                if jump {
                    velocity.on_floor = false;
                    velocity.y = SNEATLING_JUMP_HEIGHT;
                }

                if let Some(mv_amount) = movement {
                    if mv_amount != 0.0 {
                        let scaled_movement = SNEATLING_SPEED * mv_amount;
                        let new_velocity = velocity.x + scaled_movement;
                        if new_velocity > 0.0 {
                            velocity.x = new_velocity.min(SNEATLING_SPEED);
                        }
                        if new_velocity < 0.0 {
                            velocity.x = new_velocity.max(-SNEATLING_SPEED);
                        }
                    }
                }
            }
        }
    }
}
