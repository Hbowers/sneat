use amethyst::{
    core::{Transform},
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Collides, Floor, Shape, Velocity};

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Collides>,
        ReadStorage<'s, Floor>,
        ReadStorage<'s, Shape>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut velocities, collides, floors, shapes, transforms): Self::SystemData) {

        for (_can_collide, sneatling_shape, sneatling_transform, sneatling_velocity) in (&collides, &shapes, &transforms, &mut velocities).join() {
            let sneatling_x = sneatling_transform.translation().x;
            let sneatling_y = sneatling_transform.translation().y;

            //Determine if a sneatling is currently in collision with a floor entity
            let mut has_hit_floor = false;
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

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}
