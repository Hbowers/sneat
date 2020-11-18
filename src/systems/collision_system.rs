use amethyst::{
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Collider, Floor, Shape, Velocity};
use crate::helpers::point_within_range;

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Collider>,
        ReadStorage<'s, Floor>,
        ReadStorage<'s, Shape>,
        ReadStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (entities, mut velocities, collides, floors, shapes, transforms): Self::SystemData,
    ) {
        for (entity, _can_collide, collider_shape, collider_transform, collider_velocity) in
            (&entities, &collides, &shapes, &transforms, &mut velocities).join()
        {
            let collider_x = collider_transform.translation().x;
            let collider_y = collider_transform.translation().y;

            let mut collided_y = false;
            let mut collided_x_left = false;
            let mut collided_x_right = false;
            for (
                other_entity,
                _other_can_collide,
                other_collider_shape,
                other_collider_transform,
            ) in (&entities, &collides, &shapes, &transforms).join()
            {
                if entity.id() != other_entity.id() {
                    let other_collider_x = other_collider_transform.translation().x
                        - (other_collider_shape.width * 0.5);
                    let other_collider_y = other_collider_transform.translation().y
                        + (other_collider_shape.height * 0.5);

                    if points_in_contact(
                        collider_x,
                        collider_y - (collider_shape.height * 0.5),
                        other_collider_x,
                        other_collider_y,
                        0.5,
                        other_collider_shape.width,
                    ) {
                        collided_y = true
                    }
                }
            }

            //Determine if a collider is currently in collision with a floor entity
            for (_floor, floor_shape, floor_transform) in (&floors, &shapes, &transforms).join() {
                let floor_x = floor_transform.translation().x - (floor_shape.width * 0.5);
                let floor_y = floor_transform.translation().y - (floor_shape.height * 0.5);

                /* Is hitting top of floor */
                if points_in_contact(
                    collider_x,
                    collider_y - (collider_shape.height / 2.),
                    floor_x,
                    floor_y + floor_shape.height,
                    0.5,
                    floor_shape.width,
                ) {
                    collided_y = true;
                }
                /* Is hitting left side of floor */
                if points_in_contact(
                    collider_y,
                    collider_x,
                    floor_y,
                    floor_x,
                    collider_shape.width / 2.,
                    floor_shape.height + 0.9,
                ) {
                    collided_x_left = true;
                }
                /* Is hitting right side of floor */
                if points_in_contact(
                    collider_y,
                    collider_x,
                    floor_y,
                    floor_x + floor_shape.width,
                    collider_shape.width / 2.,
                    floor_shape.height + 0.9,
                ) {
                    collided_x_right = true;
                }
            }
            if collided_y {
                collider_velocity.collided_y = true;
            } else {
                collider_velocity.collided_y = false;
            }
            if collided_x_left {
                collider_velocity.collided_x_left = true;
            } else {
                collider_velocity.collided_x_left = false;
            };
            if collided_x_right {
                collider_velocity.collided_x_right = true;
            } else {
                collider_velocity.collided_x_right = false;
            };
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}

fn points_in_contact(x1: f32, y1: f32, x2: f32, y2: f32, width1: f32, width2: f32) -> bool {
    if point_within_range(y2 - width1, y2 + width1, y1) && point_within_range(x2, x2 + width2, x1) {
        true
    } else {
        false
    }
}
