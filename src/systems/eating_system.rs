use amethyst::{
    core::Hidden,
    core::Transform,
    derive::SystemDesc,
    ecs::{Entities, Join, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::Edible;
use crate::components::Shape;
use crate::components::Sneatling;
use crate::components::Collider;
use crate::types::Direction;

#[derive(SystemDesc)]
pub struct EatingSystem;

const EDIBLE_DISTANCE_X: f32 = 6.;
const EDIBLE_DISTANCE_Y: f32 = 2.;

impl<'s> System<'s> for EatingSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Edible>,
        WriteStorage<'s, Sneatling>,
        WriteStorage<'s, Hidden>,
        WriteStorage<'s, Collider>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Shape>,
    );

    fn run(
        &mut self,
        (entities, mut edibles, mut sneatlings, mut hiddens, mut colliders, transforms, shapes): Self::SystemData,
    ) {
        for (sneatling, sneatling_transform) in (&mut sneatlings, &transforms).join() {
            let sneatling_x = sneatling_transform.translation().x;
            let sneatling_y = sneatling_transform.translation().y;
                if !sneatling.is_eating || sneatling.has_eaten {
                    return;
                };

            for (entity, edible, edible_shape, edible_transform) in
                (&*entities, &mut edibles, &shapes, &transforms).join()
            {
                if edible.in_stomach {
                    return;
                };

                let edible_x = edible_transform.translation().x;
                let edible_y = edible_transform.translation().y;

                let in_range_y = sneatling_y + EDIBLE_DISTANCE_Y
                    > edible_y - edible_shape.width / 2.
                    && sneatling_y - EDIBLE_DISTANCE_Y < edible_y + edible_shape.width / 2.;

                // In range to eat
                let in_range_x: bool = match sneatling.direction {
                    Direction::Right => {
                        let sneatling_reach = sneatling_x + EDIBLE_DISTANCE_X;
                        let edible_edge = edible_x - edible_shape.width / 2.;

                        edible_edge > sneatling_x && edible_edge < sneatling_reach
                    }
                    Direction::Left => {
                        let sneatling_reach = sneatling_x - EDIBLE_DISTANCE_X;
                        let edible_edge = edible_x + edible_shape.width / 2.;

                        edible_edge < sneatling_x && edible_edge > sneatling_reach
                    }
                };

                if in_range_x && in_range_y {
                    sneatling.is_eating = false;
                    sneatling.has_eaten = true;
                    sneatling.stomach_stack += 1;
                    println!("Sneatling Stomach Stack: {}", sneatling.stomach_stack);

                    edible.in_stomach = true;
                    edible.stomach_id = sneatling.stomach_stack;
                    println!("Edible ID: {}", edible.stomach_id);

                    hiddens.insert(entity, Hidden).unwrap();
                    colliders.remove(entity);
                }
            }
        }
    }
}
