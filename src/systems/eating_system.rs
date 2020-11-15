use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::Edible;
use crate::components::Shape;
use crate::components::Sneatling;
use crate::types::Direction;

#[derive(SystemDesc)]
pub struct EatingSystem;

const EDIBLE_DISTANCE_X: f32 = 6.;
const EDIBLE_DISTANCE_Y: f32 = 2.;

impl<'s> System<'s> for EatingSystem {
    type SystemData = (
        WriteStorage<'s, Edible>,
        WriteStorage<'s, Sneatling>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Shape>,
    );

    fn run(&mut self, (mut edibles, mut sneatlings, transforms, shapes): Self::SystemData) {
        for (sneatling, sneatling_transform) in (&mut sneatlings, &transforms).join() {
            let sneatling_x = sneatling_transform.translation().x;
            let sneatling_y = sneatling_transform.translation().y;

            for (edible, edible_shape, edible_transform) in
                (&mut edibles, &shapes, &transforms).join()
            {
                if !sneatling.is_eating { return };

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
                    edible.in_stomach = true;
                    println!("HE ATE IT!")
                }
            }
        }
    }
}
