use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Tile {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
            width: width as i32,
            height: height as i32,
        }
    }

    pub fn contains(&self, point: DVec2) -> bool {
        point.x >= self.x.into()
            && point.x <= (self.x + self.width).into()
            && point.y >= self.y.into()
            && point.y <= (self.y + self.height).into()
    }

    pub fn render(&self, c: Context, gl: &mut GlGraphics, color: [f32; 4]) {
        let rectangle = rectangle::Rectangle::new(color);
        rectangle.draw(
            rectangle::centered([
                self.x as f64 + self.width as f64 / 2.0,
                self.y as f64 + self.height as f64 / 2.0,
                self.width as f64 / 2.0,
                self.height as f64 / 2.0,
            ]),
            &c.draw_state,
            c.transform,
            gl,
        );
    }
}

pub struct Map {
    pub tiles: Vec<Tile>,
    pub map: Texture,
}

impl Map {
    pub fn new() -> Self {
        let config = load_json("assets/config");

        let mut tiles = vec![];
        for tile in load_json(config["outputDir"].as_str().unwrap())["tiles"]
            .as_array()
            .unwrap()
        {
            tiles.push(Tile::new(
                tile["x"].as_f64().unwrap(),
                tile["y"].as_f64().unwrap(),
                tile["w"].as_f64().unwrap(),
                tile["h"].as_f64().unwrap(),
            ))
        }

        Self {
            tiles,
            map: Texture::from_path(
                PathBuf::from(config["mapImagePath"].as_str().unwrap()).with_extension("png"),
                &TextureSettings::new().filter(Filter::Nearest),
            )
            .unwrap(),
        }
    }

    pub fn remove_tile(&mut self, x: f64, y: f64) {
        let mut index_to_remove = None;
        for (i, tile) in self.tiles.iter().enumerate() {
            if tile.contains(DVec2::new(x, y)) {
                unsafe { ACTION = "remove" }
                index_to_remove = Some(i)
            }
        }

        if let Some(i) = index_to_remove {
            self.tiles.remove(i);
        }
    }

    pub fn render(&mut self, c: Context, gl: &mut GlGraphics, camera: &Camera, mouse: &Mouse) {
        image(&self.map, c.transform.trans(0.0, 0.0), gl);
        for tile in &self.tiles {
            let mpos = mouse_pos_relative(camera, mouse) + TILE_SIZE / 2.0;
            let color = [1.0, 0.35, 0.35, 1.0];
            if tile.contains(mpos) {
                let mut color = color.clone();
                color[1] = 0.5;
                color[2] = 0.5;
                tile.render(c, gl, color)
            } else {
                tile.render(c, gl, color)
            }
        }
    }
}

pub fn save(map: &mut Map) {
    unsafe { ACTION = "save" }

    let mut tiles = vec![];

    for tile in map.tiles.iter() {
        let json = json!({
            "x": tile.x,
            "y": tile.y,
            "w": tile.width,
            "h": tile.height
        });
        tiles.push(json)
    }

    let config = load_json("assets/config");

    write_json(
        config["outputDir"].as_str().unwrap(),
        json!({ "tiles": tiles, "meta": serde_json::from_str::<Value>(META).unwrap() }),
    )
}
