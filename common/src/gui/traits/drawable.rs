use sdl2::{render::Canvas, video::Window, pixels::Color, rect::Rect};

pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas<Window>);
}

