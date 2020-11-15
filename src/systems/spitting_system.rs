use amethyst::{
    core::Hidden,
    core::Transform,
    derive::SystemDesc,
    ecs::{Entities, Join, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::Edible;
use crate::components::Shape;
use crate::components::Sneatling;
use crate::components::SpitTravel;
use crate::types::Direction;

#[derive(SystemDesc)]
pub struct SpittingSystem;

impl<'s> System<'s> for SpittingSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Edible>,
        WriteStorage<'s, Sneatling>,
        WriteStorage<'s, Hidden>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpitTravel>,
    );

    fn run(
        &mut self,
        (entities, mut edibles, mut sneatlings, mut hiddens, transforms, mut spit_travels): Self::SystemData,
    ) {
        for (sneatling, sneatling_transform) in (&mut sneatlings, &transforms).join() {
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

                    println!("Stomach stack: {}", sneatling.stomach_stack);
                    sneatling.stomach_stack -= 1;

                    edible.in_stomach = false;
                    println!("Edible Stomach ID: {}", edible.stomach_id);
                    edible.stomach_id = -1;

                    let pos = (sneatling_x, sneatling_y);
                    let vel = match sneatling.direction {
                        Direction::Left => (-3., 4.),
                        Direction::Right => (3., 4.),
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
