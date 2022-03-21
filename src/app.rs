use graphics::grid::Grid;

use crate::prelude::*;

pub struct App {
    gl: GlGraphics,
    ui: Ui,
    map: Map,
    mouse: Mouse,
    camera: Camera,
    grid: Grid,
    undo: (bool, bool),
    current_tile: Option<Tile>,
    draw_tile: Option<Tile>,
}

impl App {
    pub fn new(gl: GlGraphics) -> Self {
        Self {
            gl,
            ui: Ui::new(vec![Text {
                id: "action",
                pos: DVec2::new(15.0, 165.0),
                text: "action: ".to_owned(),
            }]),
            map: Map::new(),
            mouse: Mouse::new(),
            camera: Camera::new(),
            grid: Grid {
                cols: 40,
                rows: 23,
                units: TILE_SIZE,
            },
            undo: Default::default(),
            current_tile: None,
            draw_tile: None,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.camera.update_viewport(args);

        self.gl.draw(self.camera.viewport, |c, gl| {
            clear([0.08235, 0.08627, 0.10588, 1.0], gl);

            self.map.render(c, gl, &self.camera, &self.mouse);

            if let Some(tile) = self.draw_tile {
                unsafe { ACTION = "place" }
                tile.render(c, gl, [1.0, 0.35, 0.35, 0.5]);
            }

            self.grid.draw(
                &Line::new([0.08235 + 0.1, 0.08627 + 0.1, 0.10588 + 0.1, 0.75], 0.17),
                &c.draw_state,
                c.transform,
                gl,
            );

            self.ui.render(c, gl, &self.camera)
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.camera.update(args);

        self.draw_tile = None;
        if let Some(tile) = self.current_tile {
            let mpos = mouse_pos_relative(&self.camera, &self.mouse);
            self.draw_tile = Some(Tile::new(
                tile.x as f64,
                tile.y as f64,
                mpos.x - tile.x as f64 + TILE_SIZE,
                mpos.y - tile.y as f64 + TILE_SIZE,
            ))
        }

        if self.mouse.button_down == MouseButton::Right {
            let mpos = mouse_pos_relative(&self.camera, &self.mouse);
            self.map.remove_tile(mpos.x, mpos.y)
        }

        if self.undo.0 && self.undo.1 {
            unsafe { ACTION = "undo" }
            self.map.tiles.pop();
            self.undo.1 = false
        }

        unsafe {
            if let Some(text) = self.ui.get_by_id("action") {
                let action = format!("action: {}", ACTION);
                text.text = action;
            }
        }
    }

    pub fn input(&mut self, args: &ButtonArgs) {
        match args.state {
            ButtonState::Press => match args.button {
                Button::Keyboard(Key::Space) => save(&mut self.map),
                Button::Keyboard(Key::R) => self.map.tiles.clear(),
                Button::Keyboard(Key::LCtrl) => self.undo.0 = true,
                Button::Keyboard(Key::Z) => self.undo.1 = true,
                Button::Mouse(MouseButton::Left) => {
                    let pos = mouse_pos_relative(&self.camera, &self.mouse);
                    self.current_tile = Some(Tile::new(pos.x, pos.y, TILE_SIZE, TILE_SIZE));
                }

                _ => (),
            },
            ButtonState::Release => match args.button {
                Button::Mouse(MouseButton::Left) => {
                    let mpos = mouse_pos_relative(&self.camera, &self.mouse);
                    if let Some(tile) = self.current_tile {
                        self.map.tiles.push(Tile::new(
                            tile.x as f64,
                            tile.y as f64,
                            mpos.x - tile.x as f64 + TILE_SIZE,
                            mpos.y - tile.y as f64 + TILE_SIZE,
                        ));
                    }
                    self.current_tile = None;
                }
                _ => (),
            },
        }

        self.camera.input(args);
        self.mouse.update(args);
    }

    pub fn cursor(&mut self, args: &[f64; 2]) {
        self.mouse
            .set_mouse_pos([args[0] / self.camera.upscale, args[1] / self.camera.upscale])
    }
}

pub fn mouse_pos_relative(camera: &Camera, mouse: &Mouse) -> DVec2 {
    DVec2::new(
        round_to(
            (mouse.pos.x - camera.pos.x / camera.upscale) - TILE_SIZE,
            TILE_SIZE,
        )
        .abs(),
        round_to(
            (mouse.pos.y - -camera.pos.y / camera.upscale) - TILE_SIZE,
            TILE_SIZE,
        )
        .abs(),
    )
}
