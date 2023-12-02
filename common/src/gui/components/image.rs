use std::path::Path;

use sdl2::{image::LoadTexture, rect::Rect};

use crate::gui::{geometry::{UDim2, UDim}, Drawable};

pub struct Image {
    position: UDim2,
    size: UDim2,
    filename: String,
}

impl Image {
    pub fn new(position: UDim2, size: UDim2, path: &str) -> Self {
        Image {position, size, filename: String::from(path)}
    }
}

impl Drawable for Image {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let tc = canvas.texture_creator();
        let texture = tc.load_texture(&self.filename).unwrap();

        canvas.copy(&texture, None, Rect::new(self.position.x.as_position(), self.position.y.as_position(), self.size.x.as_size(), self.size.y.as_size())).unwrap();
    }
}