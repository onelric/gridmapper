use std::path::PathBuf;

use glutin_window::GlutinWindow;
use graphics::clear;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    AdvancedWindow, EventSettings, Events, RenderEvent, UpdateEvent, Window, WindowSettings,
};

pub struct SaveWindow {
    gl: GlGraphics,
    output_path: PathBuf,
    window: GlutinWindow,
}

impl SaveWindow {
    pub fn new(opengl: OpenGL) -> Self {
        let gl = GlGraphics::new(opengl);

        Self {
            gl,
            output_path: PathBuf::default(),
            window: WindowSettings::new("path", [960, 540])
                .graphics_api(opengl)
                .exit_on_esc(true)
                .build()
                .unwrap(),
        }
    }

    pub fn run(mut self) -> Self {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            println!("hello from other window");
            if self.window.should_close() {
                self.window.hide()
            }

            // if let Some(args) = e.render_args() {
            //     self.gl.draw(args.viewport(), |c, gl| clear([1.0; 4], gl));
            // }
            // if let Some(args) = e.update_args() {
            // }
        }

        self
    }
}
