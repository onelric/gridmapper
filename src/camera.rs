use crate::prelude::{ButtonArgs, DVec2, Key, UpdateArgs};
use graphics::Viewport;
use piston::RenderArgs;

#[derive(Default)]
pub struct InputDirection {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

pub struct Camera {
    pub pos: DVec2,
    vel: DVec2,
    direction: DVec2,
    input: InputDirection,
    pub viewport: Viewport,
    pub upscale: f64,
}

impl Camera {
    pub fn new() -> Self {
        let viewport = Viewport {
            rect: [0, 0, 1280, 720],
            draw_size: [1280, 720],
            window_size: [1280.0, 720.0],
        };

        Self {
            pos: DVec2::ZERO,
            vel: DVec2::ZERO,
            direction: DVec2::ZERO,
            input: InputDirection::default(),
            viewport,
            upscale: viewport.window_size[0] / 320.0,
        }
    }

    fn movement(&mut self, dt: f64) {
        self.direction = DVec2::ZERO;
        if self.input.up && !self.input.down {
            self.direction.y = -1.0
        } else if self.input.down && !self.input.up {
            self.direction.y = 1.0
        }

        if self.input.left && !self.input.right {
            self.direction.x = 1.0
        } else if self.input.right && !self.input.left {
            self.direction.x = -1.0
        }

        self.vel = self
            .vel
            .lerp(self.direction.normalize_or_zero() * 350.0 * dt, 10.0 * dt);
        self.pos += self.vel;
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.movement(args.dt);
    }

    pub fn input(&mut self, args: &ButtonArgs) {
        self.direction = DVec2::ZERO;
        match args.state {
            piston::ButtonState::Press => match args.button {
                piston::Button::Keyboard(Key::W) => self.input.up = true,
                piston::Button::Keyboard(Key::S) => self.input.down = true,
                piston::Button::Keyboard(Key::D) => self.input.right = true,
                piston::Button::Keyboard(Key::A) => self.input.left = true,
                _ => (),
            },
            piston::ButtonState::Release => match args.button {
                piston::Button::Keyboard(Key::W) => self.input.up = false,
                piston::Button::Keyboard(Key::S) => self.input.down = false,
                piston::Button::Keyboard(Key::D) => self.input.right = false,
                piston::Button::Keyboard(Key::A) => self.input.left = false,
                _ => (),
            },
        }
    }

    pub fn update_viewport(&mut self, args: &RenderArgs) {
        self.upscale = self.viewport.window_size[0] / 320.0;

        let mut viewport = args.viewport();
        viewport.draw_size[0] =
            viewport.window_size[0] as u32 * viewport.window_size[0] as u32 / 320;
        viewport.draw_size[1] =
            viewport.window_size[1] as u32 * viewport.window_size[1] as u32 / 180;

        viewport.rect[0] = self.pos.x as i32;
        viewport.rect[1] = self.pos.y as i32;

        self.viewport = viewport;
    }
}
