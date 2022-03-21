#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

extern crate const_format;

extern crate glam;
extern crate glutin_window;
extern crate graphics;
extern crate num;
extern crate opengl_graphics;
extern crate piston;
extern crate serde;
extern crate serde_json;

mod app;
mod camera;
mod json;
mod map;
mod mouse;
mod prelude;
mod saving;
mod ui;
mod utilities;

use const_format::formatcp;
use piston::EventLoop;

const TILE: i32 = 8;
pub const META: &'static str = formatcp!(
    r#"
    {{
        "tileWidth": {TILE},
        "tileHeight": {TILE}
    }}
    "#,
);

pub const TILE_SIZE: f64 = TILE as f64;

pub static mut ACTION: &'static str = "none";

fn main() {
    use crate::prelude::*;

    let opengl = OpenGL::V4_5;

    let mut window: GlutinWindow = WindowSettings::new("GridMapper", [1280, 720])
        .graphics_api(opengl)
        .vsync(false)
        .build()
        .unwrap();

    // let save_window = SaveWindow::new(opengl).run();
    // std::mem::drop(save_window);

    let mut app = App::new(GlGraphics::new(opengl));
    let mut events = Events::new(EventSettings::new().max_fps(360));
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.button_args() {
            app.input(&args);
        }

        if let Some(args) = e.mouse_cursor_args() {
            app.cursor(&args);
        }
    }
}
