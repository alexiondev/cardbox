use core::cmp::max;

use super::{SCREEN_WIDTH, SCREEN_HEIGHT};

#[derive(Clone, Copy)]
pub struct UDim {
    pub relative: f32,
    pub offset: i32,
    scale: u32,
}

impl UDim {
    fn new(relative: f32, offset: i32, scale: u32) -> Self {
        UDim { relative, offset, scale }
    }

    pub fn as_position(&self) -> i32 {
        (self.scale as f32 * self.relative) as i32 + self.offset
    }

    pub fn as_size(&self) -> u32 {
        max(
            (self.scale as f32 * self.relative) as i32 + self.offset,
            0
        ) as u32
    }
}

#[derive(Clone, Copy)]
pub struct UDim2 {
    pub x: UDim,
    pub y: UDim,
}

impl UDim2 {
    pub fn new(x_relative: f32, x_offset: i32, y_relative: f32, y_offset: i32) -> Self {
        UDim2 {
            x: UDim::new(x_relative, x_offset, SCREEN_WIDTH),
            y: UDim::new(y_relative, y_offset, SCREEN_HEIGHT),
        }
    }
}
