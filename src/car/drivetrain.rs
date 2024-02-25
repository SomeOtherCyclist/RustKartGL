use crate::car::gearbox;

#[derive(Default)]
pub struct Drivetrain {
    pub mass: f32,

    pub gearbox: gearbox::Gearbox
}

impl Drivetrain {

}