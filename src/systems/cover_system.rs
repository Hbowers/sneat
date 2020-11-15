use amethyst::{
    core::{Transform},
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Coverable, Shape, Covers};

pub struct CoverSystem;

impl<'s> System<'s> for CoverSystem {
    type SystemData = (
        WriteStorage<'s, Coverable>,
        ReadStorage<'s, Covers>,
        ReadStorage<'s, Shape>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut coverables, covers, shapes, transforms): Self::SystemData) {

        for (coverable, coverable_shape, coverable_transform) in (&mut coverables, &shapes, &transforms).join() {
            let mut covered_amount = 0.0;

            for (_cover, cover_shape, cover_transform) in (&covers, &shapes, &transforms).join() {
                let overlap = overlap_percentage(
                    coverable_transform.translation().x - (coverable_shape.width * 0.5),
                    coverable_transform.translation().x + (coverable_shape.width * 0.5),
                    coverable_transform.translation().y - (coverable_shape.height * 0.5),
                    coverable_transform.translation().y + (coverable_shape.height * 0.5),
                    cover_transform.translation().x - (cover_shape.width * 0.5),
                    cover_transform.translation().x + (cover_shape.width * 0.5),
                    cover_transform.translation().y - (cover_shape.height * 0.5),
                    cover_transform.translation().y + (cover_shape.height * 0.5),
                );
                covered_amount += overlap;
            }

            coverable.covered_amount = covered_amount;
            println!("Entity covered amount: {:?}", coverable.covered_amount);
        }
    }
}

fn overlap_percentage(xa1: f32, xa2: f32, ya1: f32, ya2: f32, xb1: f32, xb2: f32, yb1: f32, yb2: f32) -> f32 {
    // x >= left && x <= right && y >= bottom && y <= top) {
    //TODO: https://stackoverflow.com/questions/9324339/how-much-do-two-rectangles-overlap
    let naught: f32 = 0.0;
    let si = naught.max(xa2.min(xb2) - xa1.max(xb1)) * naught.max(ya2.min(yb2) -ya1.max(yb1));
    let a1 = (xa2 - xa1) * (ya2 - ya1);
    clamp(0.0, 100.0,(si / (a1 - si)) * 100.0)
}

fn clamp(min: f32, max: f32, mut value: f32) -> f32 {
    assert!(min <= max);
    if value < min {
        value = min;
    }
    if value > max {
        value = max;
    }
    value
}