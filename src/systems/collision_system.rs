use amethyst::{
    core::{Transform},
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Collider,Collidee, Floor, Shape, Velocity};
use crate::helpers::point_within_range;

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Collider>,
        ReadStorage<'s, Collidee>,
        ReadStorage<'s, Floor>,
        ReadStorage<'s, Shape>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut velocities, collides, collidees, floors, shapes, transforms): Self::SystemData) {

        for (_can_collide, collider_shape, collider_transform, collider_velocity) in (&collides, &shapes, &transforms, &mut velocities).join() {
            let collider_x = collider_transform.translation().x;
            let collider_y = collider_transform.translation().y;


            let mut collided = false;
            for (_other_can_collide, other_collider_shape, other_collider_transform) in (&collides, &shapes, &transforms).join() {
                // if _other_can_collide.can_collide {

                // }
                let other_collider_x = other_collider_transform.translation().x - (other_collider_shape.width * 0.5);
                let other_collider_y = other_collider_transform.translation().y + (other_collider_shape.height * 0.5);

                if points_in_contact(
                    collider_x,
                    collider_y + (collider_shape.height * 0.5),
                 collider_shape.width,
                    other_collider_x,
                    other_collider_y,
                other_collider_shape.width) {
                    collided = true
                }
            }

            //Determine if a collider is currently in collision with a floor entity
            for (_floor, floor_shape, floor_transform) in (&floors, &shapes, &transforms).join() {
                let floor_x = floor_transform.translation().x - (floor_shape.width * 0.5);
                let floor_y = floor_transform.translation().y - (floor_shape.height * 0.5);

                if point_in_rect(
                    collider_x,
                    collider_y,
                    floor_x - collider_shape.width /2.,
                    floor_y - collider_shape.height /2.,
                    floor_x + floor_shape.width + collider_shape.width /2.,
                    floor_y + floor_shape.height + collider_shape.height /2.,
                ) {
                    collided = true;
                }
            }
            if collided {
                collider_velocity.collided = true;
            } else {
                collider_velocity.collided = false;
            }
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}

fn points_in_contact(x1: f32, y1: f32, width1: f32, x2: f32, y2: f32, width2: f32) -> bool {
    if
        point_within_range(y2, y1 + 1., y1) &&
        point_within_range(x2, x2 + width2, x1)
        // x1 > x2 && x1 + width1 < x2 + width2 ||
        // x1 + width1 > x2 && x1 < x2
    {
        println!("Collider pos: x{:?}, y{:?}", x1, y1);
        println!("other collider pos: x{:?}, y{:?}, width{:?}", x2, y2, width2);
        true
    } else {
        false
    }
}