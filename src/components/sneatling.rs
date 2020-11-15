use crate::types::{Direction,SneatlingAnimState};
use amethyst::ecs::{Component, DenseVecStorage};


pub struct Sneatling {
    pub is_eating: bool,
    pub is_spitting: bool,
    pub has_spat: bool,
    pub has_eaten: bool,
    pub stomach_stack: i32,
    pub direction: Direction,
    pub sneatling_anim_state: SneatlingAnimState
}

impl Sneatling {
    pub fn new() -> Sneatling {
        Sneatling {
            is_eating: false,
            is_spitting: false,
            has_spat: false,
            has_eaten: false,
            stomach_stack: 0,
            direction: Direction::Left,
            sneatling_anim_state: SneatlingAnimState::Idle
        }
    }
}

impl Component for Sneatling {
    type Storage = DenseVecStorage<Self>;
}
