use crate::components::Animation;
use amethyst::{
    core::Time,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

pub struct AnimationSystem;

impl<'s> System<'s> for AnimationSystem {
    type SystemData = (
        ReadStorage<'s, Animation>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Time>,
    );

    fn run(&mut self, (animations, mut sprite_renders, time): Self::SystemData) {
        for (animation, sprite) in (&animations, &mut sprite_renders).join() {
            let elapsed_time = time.frame_number();
            let frame = (elapsed_time / animation.frame_duration) as i32 % animation.frames;

            sprite.sprite_number = animation.first_sprite_index + frame as usize;
        }
    }
}
