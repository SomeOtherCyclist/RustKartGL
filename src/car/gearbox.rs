pub struct Gearbox {
    pub mass: f32,

    pub final_drive: f32,
    reverse: f32,
    pub gears: Vec<f32>,
    pub gear: i8,
    pub ratio: f32,
}

impl Default for Gearbox {
    fn default() -> Self {
        Gearbox {
            mass: 0.0,

            final_drive: 1.0,
            reverse: -1.0,
            gears: vec![0.0; 6],
            gear: 0,
            ratio: 1.0,
        }
    }
}

impl Gearbox {
    pub fn ratio(mut self) {
        self.ratio = match self.gear {
            -1 => self.reverse,
            0..=127 => self.gears[self.gear as usize],
            _ => 1.0
        }
    }
}