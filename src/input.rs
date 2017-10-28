use glutin::{ElementState, KeyboardInput};

pub struct Input {
    pub move_north: bool,
    pub move_south: bool,
    pub move_west:  bool,
    pub move_east:  bool,
    pub attack: bool,
}

impl Input {
    pub fn new() -> Self {
        Input{
            move_north: false,
            move_south: false,
            move_west:  false,
            move_east:  false,
            attack:     false,
        }
    }

    pub fn keyboard_input(&mut self, input: &KeyboardInput) {
        let pressed = match input.state {
            ElementState::Pressed => true,
            ElementState::Released => false,
        };
        match input.scancode {
            0x11 /*   W   */ => self.move_north = pressed,
            0x1F /*   S   */ => self.move_south = pressed,
            0x1E /*   A   */ => self.move_west  = pressed,
            0x20 /*   D   */ => self.move_east  = pressed,
            0x39 /* space */ => self.attack     = pressed,
            _ => (),
        }
    }
}
