use crate::prelude::*;

pub struct Mouse {
    pub pos: DVec2,
    pub button_down: MouseButton,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            pos: DVec2::ZERO,
            button_down: MouseButton::Unknown,
        }
    }

    pub fn update(&mut self, args: &ButtonArgs) {
        match args.state {
            ButtonState::Press => match args.button {
                Button::Mouse(MouseButton::Left) => self.button_down = MouseButton::Left,
                Button::Mouse(MouseButton::Right) => self.button_down = MouseButton::Right,
                _ => (),
            },
            ButtonState::Release => match args.button {
                Button::Mouse(MouseButton::Left) | Button::Mouse(MouseButton::Right) => {
                    self.button_down = MouseButton::Unknown
                }
                _ => (),
            },
        }
    }

    pub fn set_mouse_pos(&mut self, pos: [f64; 2]) {
        self.pos = DVec2::new(pos[0], pos[1]);
    }
}
