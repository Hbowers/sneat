use amethyst::{
    core::Hidden,
    core::Transform,
    derive::SystemDesc,
    ecs::{Entities, Join, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::Sneatling;
use crate::components::Animation;
use crate::types::SneatlingAnimState;

#[derive(SystemDesc)]
pub struct AnimationChangingSystem;

const ANIMATION_DURATION:u64 = 10;

impl<'s> System<'s> for AnimationChangingSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Animation>,
        ReadStorage<'s, Sneatling>,
    );

    fn run(
        &mut self,
        (entities, mut animations, sneatlings): Self::SystemData,
    ) {
        for (entity, sneatling) in (&entities, &sneatlings).join() {
            match &sneatling.sneatling_anim_state {
               SneatlingAnimState::Idle=> {
                    let new_animation = Animation {
                        frames: 44,
                        frame_duration: ANIMATION_DURATION,
                        first_sprite_index: 0,
                    };
                    animations.insert(entity, new_animation);
                },
                SneatlingAnimState::WalkingRight => {
                    let new_animation = Animation {
                        frames: 14,
                        frame_duration: ANIMATION_DURATION,
                        first_sprite_index: 45,
                    };
                    animations.insert(entity, new_animation);
                },
                SneatlingAnimState::WalkingLeft => {
                    let new_animation = Animation {
                        frames: 14,
                        frame_duration: ANIMATION_DURATION,
                        first_sprite_index: 45,
                    };
                    animations.insert(entity, new_animation);
                },
                SneatlingAnimState::Jumping => {
                    let new_animation = Animation {
                        frames: 4,
                        frame_duration: ANIMATION_DURATION,
                        first_sprite_index: 59,
                    };
                    animations.insert(entity, new_animation);
                },
                SneatlingAnimState::Eating => {
                    let new_animation = Animation {
                        frames: 10,
                        frame_duration: ANIMATION_DURATION,
                        first_sprite_index: 79,
                    };
                    animations.insert(entity, new_animation);
                },
                SneatlingAnimState::Spitting => {
                    let new_animation = Animation {
                        frames: 11,
                        frame_duration: ANIMATION_DURATION,
                        first_sprite_index: 68,
                    };
                    animations.insert(entity, new_animation);
                },
                _ => {
                    let new_animation = Animation {
                        frames: 44,
                        frame_duration: ANIMATION_DURATION,
                        first_sprite_index: 45,
                    };
                    animations.insert(entity, new_animation);
                },
            }
        }
    }
}
