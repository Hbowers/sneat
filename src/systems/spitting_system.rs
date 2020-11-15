use amethyst::{
    core::Hidden,
    core::Transform,
    derive::SystemDesc,
    ecs::{Entities, Join, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::Edible;
use crate::components::Sneatling;
use crate::components::SpitTravel;
use crate::components::Velocity;
use crate::types::Direction;

#[derive(SystemDesc)]
pub struct SpittingSystem;

const SPIT_BOOST_MOVING_X: f32 = 1.2;
const SPIT_BOOST_MOVING_Y: f32 = 2.3;
const SPIT_SPEED_STANDING_X: f32 = 0.6;
const SPIT_SPEED_STANDING_Y: f32 = 5.3;
const X_OFFSET: f32 = 0.3;
const Y_OFFSET: f32 = 2.5;

impl<'s> System<'s> for SpittingSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Edible>,
        WriteStorage<'s, Sneatling>,
        WriteStorage<'s, Hidden>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpitTravel>,
        ReadStorage<'s, Velocity>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut edibles,
            mut sneatlings,
            mut hiddens,
            transforms,
            mut spit_travels,
            velocities,
        ): Self::SystemData,
    ) {
        for (sneatling, sneatling_transform, sneatling_velocity) in
            (&mut sneatlings, &transforms, &velocities).join()
        {
            let sneatling_x = sneatling_transform.translation().x;
            let sneatling_y = sneatling_transform.translation().y;
            for (entity, edible) in (&*entities, &mut edibles).join() {
                /* It's in the stomach and at the top of the stack */
                if sneatling.stomach_stack == 0
                    || !sneatling.is_spitting
                    || sneatling.has_spat
                    || sneatling.has_eaten
                    || sneatling.is_eating
                {
                    return;
                }

                if edible.in_stomach && edible.stomach_id == sneatling.stomach_stack {
                    hiddens.remove(entity);

                    sneatling.stomach_stack -= 1;

                    edible.in_stomach = false;
                    edible.stomach_id = -1;

                    let pos = (sneatling_x + X_OFFSET, sneatling_y + Y_OFFSET);
                    let vel = match sneatling.direction {
                        Direction::Left => {
                            let x_vel = if sneatling_velocity.x != 0. {
                                sneatling_velocity.x - SPIT_BOOST_MOVING_X
                            } else {
                                -SPIT_SPEED_STANDING_X
                            };
                            let y_vel = if sneatling_velocity.y > 0. {
                                sneatling_velocity.y + SPIT_BOOST_MOVING_Y
                            } else {
                                SPIT_SPEED_STANDING_Y
                            };
                            (x_vel, y_vel)
                        }
                        Direction::Right => {
                            let x_vel = if sneatling_velocity.x != 0. {
                                sneatling_velocity.x + SPIT_BOOST_MOVING_X
                            } else {
                                SPIT_SPEED_STANDING_X
                            };
                            let y_vel = if sneatling_velocity.y > 0. {
                                sneatling_velocity.y + SPIT_BOOST_MOVING_Y
                            } else {
                                SPIT_SPEED_STANDING_Y
                            };
                            (x_vel, y_vel)
                        }
                    };

                    // FIXME: unwrap is dirty
                    spit_travels
                        .insert(entity, SpitTravel::new(pos, vel))
                        .unwrap();

                    sneatling.is_spitting = false;
                    sneatling.has_spat = true;
                }
            }
        }
    }
}
