pub type Point = (f32, f32);

pub enum Direction {
    Left,
    Right
}

pub enum SneatlingAnimState {
    Idle,
    WalkingLeft,
    WalkingRight,
    Eating,
    Jumping,
    Spitting,
}