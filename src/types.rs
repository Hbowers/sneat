pub type Point = (f32, f32);

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum SneatlingAnimState {
    Idle,
    WalkingLeft,
    WalkingRight,
    Eating,
    Jumping,
    Spitting,
}
