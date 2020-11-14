use amethyst::{
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::components::{Sneatling, Floor, Shape, Velocity};

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Sneatling>,
        ReadStorage<'s, Floor>,
        ReadStorage<'s, Shape>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut velocities, sneatlings, floors, shapes, transforms): Self::SystemData) {

        for (sneatling, sneatling_shape, sneatling_transform, sneatling_velocity) in (&sneatlings, &shapes, &transforms, &mut velocities).join() {
            let sneatling_x = sneatling_transform.translation().x;
            let sneatling_y = sneatling_transform.translation().y;

            

            for (floor, floor_shape, floor_transform) in (&floors, &shapes, &transforms).join() {
                println!("checking collision, sneatling: {:?}, {:?}, floor: {:?}, {:?}", sneatling_transform.translation().x, sneatling_transform.translation().y, floor_transform.translation().x, floor_transform.translation().y);
                let floor_x = floor_transform.translation().x - (floor_shape.width + 5.0);
                let floor_y = floor_transform.translation().y - (floor_shape.height + 5.0);

                if point_in_rect(
                    sneatling_x,
                    sneatling_y,
                    floor_x - sneatling_shape.width,
                    floor_y - sneatling_shape.height,
                    floor_x + floor_shape.width + sneatling_shape.width,
                    floor_y + floor_shape.height + sneatling_shape.height,
                ) {
                    sneatling_velocity.on_floor = false;
                }
            }
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    println!("point in rect");
    x >= left && x <= right && y >= bottom && y <= top
}
