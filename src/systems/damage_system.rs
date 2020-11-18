use amethyst::{
    core::{Time, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{Coverable, Health, Sneatling};

pub struct DamageSystem;

impl<'s> System<'s> for DamageSystem {
    type SystemData = (
        ReadStorage<'s, Coverable>,
        WriteStorage<'s, Health>,
        Read<'s, Time>,
    );

    fn run(&mut self, (coverables, mut health, time): Self::SystemData) {
        for (coverable, health) in (&coverables, &mut health).join() {
            if !coverable.in_cover() && coverable.time_out_of_cover > 1.5 {
                if health.value > 0.0 {
                    println!("Damaging, health remaining: {:?}", health.value);
                    health.value -= time.delta_seconds();
                } else {
                    //FIXME destroy object OR migrate to state
                }
            }
        }
    }
}
