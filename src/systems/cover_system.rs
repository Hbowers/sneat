use amethyst::{
    core::{Transform},
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Sneatling, Shape, Cover};

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        ReadStorage<'s, Sneatling>,
        ReadStorage<'s, Cover>
        ReadStorage<'s, Shape>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (sneatlings, covers, shapes, transforms): Self::SystemData) {

        for (_sneatling, sneatling_shape, sneatling_transform) in (&sneatlings, &shapes, &transforms).join() {
            let sneatling_x = sneatling_transform.translation().x;
            let sneatling_y = sneatling_transform.translation().y;

            for (_cover, cover_shape, cover_transform) in (&covers, &shapes, &transforms).join() {
                let cover_x = cover_transform.translation().x - (cover_shape.width);
                let cover_y = cover_transform.translation().y - (cover_shape.height);


            }

            //Determine if a sneatling is currently in collision with a floor entity
            for (_floor, floor_shape, floor_transform) in (&floors, &shapes, &transforms).join() {
                let floor_x = floor_transform.translation().x - (floor_shape.width * 0.5);
                let floor_y = floor_transform.translation().y - (floor_shape.height * 0.5);

                if point_in_rect(
                    sneatling_x,
                    sneatling_y,
                    floor_x - sneatling_shape.width /2.,
                    floor_y - sneatling_shape.height /2.,
                    floor_x + floor_shape.width + sneatling_shape.width /2.,
                    floor_y + floor_shape.height + sneatling_shape.height /2.,
                ) {
                    println!("colliding");
                    has_hit_floor = true;
                }
            }
            if has_hit_floor {
                sneatling_velocity.on_floor = true;
            } else {
                sneatling_velocity.on_floor = false;
            }
        }
    }
}

fn overlbp_percentage(xa1: f32, xa2: f32, ya1: f32, ya2: f32, xb1: f32, xb2: f32, yb1: f32, yb2: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
    //TODO: https://stackoverflow.com/questions/9324339/how-much-do-two-rectangles-overlap

}
