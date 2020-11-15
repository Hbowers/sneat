use amethyst::{
    core::Hidden,
    core::Transform,
    derive::SystemDesc,
    ecs::{Entities, Join, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::Edible;
use crate::components::Shape;
use crate::components::Sneatling;
use crate::components::Velocity;
use crate::components::SpitTravel;
use crate::types::Direction;
use crate::components::Collider;

#[derive(SystemDesc)]
pub struct SpitTravelSystem;

impl<'s> System<'s> for SpitTravelSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Hidden>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpitTravel>,
        WriteStorage<'s, Collider>,
    );

    fn run(
        &mut self,
        (entities, mut hiddens,mut velocities, mut transforms, mut spit_travels, mut colliders): Self::SystemData,
    ) {
        let mut to_remove = Vec::new();
        for (entity, spit_item_transform, spit_travel, mut velocity) in (&entities, &mut transforms, &spit_travels, &mut velocities).join() {
            to_remove.push(entity);

            spit_item_transform.set_translation_x(spit_travel.position.0);
            spit_item_transform.set_translation_y(spit_travel.position.1);

            velocity.x = spit_travel.velocity.0;
            velocity.y = spit_travel.velocity.1;
        }
        for entity in to_remove {
            hiddens.remove(entity);
            spit_travels.remove(entity);
            colliders.insert(entity, Collider::new()).unwrap();
        }
    }
}
