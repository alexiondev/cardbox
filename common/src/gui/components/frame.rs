use sdl2::{pixels::Color, rect::Rect};

use crate::gui::{geometry::UDim2, GuiObject, Drawable};

pub struct Frame {
    position: UDim2,
    size: UDim2,
}

impl Frame {
    pub fn new(position: UDim2, size: UDim2) -> Self {
        Self {
            position, size
        }
    }
}
impl Drawable for Frame {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(Color::RGB(255, 255, 0));
        canvas.fill_rect(Rect::new(
            self.position.x.as_position(), 
            self.position.y.as_position(), 
            self.size.x.as_size(), 
            self.size.y.as_size())).unwrap();
    }
}