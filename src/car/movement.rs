use crate::car::input;

#[derive(Default)]
pub struct Movement {
    pub input: input::Input,

    pub direction: [f32; 3],
    pub velocity: [f32; 3],
}

impl Movement {
    
}