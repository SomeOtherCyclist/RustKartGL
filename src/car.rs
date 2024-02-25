mod movement;
mod engine;
mod drivetrain;
mod body;
mod tyres;
pub mod gearbox;
pub mod input;

#[derive(Default)]
pub struct Car {
    pub is_player: bool,
    pub mass: f32,

    pub movement: movement::Movement,

    pub engine: engine::Engine,
    pub drivetrain: drivetrain::Drivetrain,
    pub body: body::Body,
    pub tyres: tyres::Tyres
}

impl Car {
    pub fn total_mass(mut self) {
        self.mass = 
            self.engine.mass +
            self.drivetrain.mass +
            self.drivetrain.gearbox.mass +
            self.body.mass +
            self.tyres.mass
    }

    pub fn player(mut self) -> Car {
        self.is_player = true;
        self
    }

    pub fn update (mut self) {

    }

    pub fn gear_up(mut self) {
        if self.drivetrain.gearbox.gear < self.drivetrain.gearbox.gears.len() as i8{
            self.drivetrain.gearbox.gear += 1
        }
    }

    pub fn gear_down(mut self) {
        // Don't allow shifting below reverse nor if speed is above ~50km/h
        if (self.drivetrain.gearbox.gear > -2) && self.movement.velocity[0] < 13.88 {
            self.drivetrain.gearbox.gear -= 1
        }
    }
}