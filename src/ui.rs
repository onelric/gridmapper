use opengl_graphics::GlyphCache;

use crate::prelude::{
    text, Camera, Context, DVec2, Filter, GlGraphics, TextureSettings, Transformed,
};

#[derive(Clone)]
pub struct Text {
    pub id: &'static str,
    pub pos: DVec2,
    pub text: String,
}

pub struct Ui {
    text: Vec<Text>,
    glyphs: GlyphCache<'static>,
}

impl Ui {
    pub fn new(text: Vec<Text>) -> Self {
        let glyphs = GlyphCache::new(
            "assets/font.ttf",
            (),
            TextureSettings::new().filter(Filter::Nearest),
        )
        .unwrap();

        Self { text, glyphs }
    }

    pub fn get_by_id(&mut self, id: &str) -> Option<&mut Text> {
        for text in self.text.iter_mut() {
            if text.id == id {
                return Some(text);
            }
        }
        None
    }

    pub fn render(&mut self, c: Context, gl: &mut GlGraphics, camera: &Camera) {
        self.text.iter_mut().for_each(|text| {
            fn draw(
                camera: &Camera,
                text: &Text,
                glyphs: &mut GlyphCache,
                c: Context,
                gl: &mut GlGraphics,
                color: [f32; 4],
            ) {
                text::Text::new_color(color, 10)
                    .draw(
                        text.text.as_str(),
                        glyphs,
                        &c.draw_state,
                        c.transform.trans(
                            text.pos.x - camera.pos.x / camera.upscale,
                            text.pos.y + camera.pos.y / camera.upscale,
                        ),
                        gl,
                    )
                    .unwrap()
            }

            draw(camera, text, &mut self.glyphs, c, gl, [0.0, 0.0, 0.0, 1.0]);
            draw(
                camera,
                &{
                    let mut t = text.clone();
                    t.pos.x = text.pos.x + 1.0;
                    t.pos.y = text.pos.y + 1.0;
                    t
                },
                &mut self.glyphs,
                c,
                gl,
                [1.0; 4],
            );
        });
    }
}
