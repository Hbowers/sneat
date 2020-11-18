use amethyst::{
    core::{Time, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{CameraFocus, Sneatling};

pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        ReadStorage<'s, Sneatling>,
        ReadStorage<'s, CameraFocus>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (sneatlings, focus, mut transforms): Self::SystemData) {
        let mut sneatling_x = 0.;

        for (_sneatling, transform) in (&sneatlings, &transforms).join() {
            sneatling_x = transform.translation().x;
        }

        for (_focus, transform) in (&focus, &mut transforms).join() {
            transform.set_translation_x(sneatling_x);
        }
    }
}
