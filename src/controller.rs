use gilrs::{EventType, Axis::*, Button::*};

#[derive(Default, Copy, Clone, PartialEq)]
pub struct ControllerEvent {
    pub stick_l: (f32, f32),
    pub stick_r: (f32, f32),
    pub trigger_l: f32,
    pub trigger_r: f32,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub north: bool,
    pub south: bool,
    pub east: bool,
    pub west: bool,
    pub select: bool,
    pub start: bool
}

impl ControllerEvent {
    pub fn event(mut self: ControllerEvent, event: EventType) -> ControllerEvent {
        match event {
            EventType::ButtonPressed(button, ..) => {
                match button {
                    DPadUp =>           self.up = true,
                    DPadDown =>         self.down = true,
                    DPadLeft =>         self.left = true,
                    DPadRight =>        self.right = true,
                    North =>            self.north = true,
                    South =>            self.south = true,
                    East =>             self.east = true,
                    West =>             self.west = true,
                    Select =>           self.select = true,
                    Start =>            self.start = true,
                    _ => ()
                }
            },
            EventType::AxisChanged(axis, value, ..) => {
                match axis {
                    LeftStickX =>       self.stick_l.0 = value,
                    LeftStickY =>       self.stick_l.1 = value,
                    RightStickX =>      self.stick_r.0 = value,
                    RightStickY =>      self.stick_r.1 = value,
                    _ => ()
                }
            },
            EventType::ButtonChanged(button, value, .. ) => {
                match button {
                    LeftTrigger2 =>     self.trigger_l = value,
                    RightTrigger2 =>    self.trigger_r = value,
                    _ => ()
                }
            },
            _ => ()
        }
        self
    }
}