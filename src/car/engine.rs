#[derive(Default)]
pub struct Engine {
    pub mass: f32,
    pub layout: Layout,
    pub aspiration: Aspiration,
    pub displacement: f32,

    pub throttle: f32,
    pub clutch: f32,
}

#[derive(Default)]
#[allow(unused)]
pub enum Layout {
    #[default] Inline,
    V,
    W3,
    W4,
    Flat,
    Boxer
}

#[derive(Default)]
#[allow(unused)]
pub enum Aspiration {
    #[default] Natural,
    Turbo,
    Turbo2,
    Turbo3,
    Turbo4,
    CSC,
    DSC,
    CSC2,
    DSC2,
    CSCTurbo,
    DSCTurbo
}