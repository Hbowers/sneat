use amethyst::{
    ecs::{Component, DenseVecStorage},
};

pub struct  CameraFocus {
}

impl CameraFocus {
    pub fn new() -> CameraFocus {
        CameraFocus {
        }
    }
}

impl Component for CameraFocus {
    type Storage = DenseVecStorage<Self>;
}
