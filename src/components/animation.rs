use amethyst::ecs::{Component, DenseVecStorage};
pub struct Animation {
    pub frames: i32,
    pub frame_duration: u64,
    pub first_sprite_index: usize,
  }
  
  impl Component for Animation {
    // The storage type of the Animation component
    type Storage = DenseVecStorage<Self>;
  }