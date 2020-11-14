use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

pub use crate::components::Sneatling;

#[derive(SystemDesc)]
pub struct SneatlingMovementSystem;

const SNEATLING_SPEED: f32 = 1.2;

impl<'s> System<'s> for SneatlingMovementSystem{
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Sneatling>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, sneatlings, input): Self::SystemData){
        for (sneatling, transform) in (&sneatlings, &mut transforms).join(){
            let movement = input.axis_value("player_1_walk");
            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    println!("Player moving {}", mv_amount);
                    let scaled_movement = SNEATLING_SPEED * mv_amount;
                    transform.prepend_translation_x(scaled_movement);
                }
            }
        }
    }
}
