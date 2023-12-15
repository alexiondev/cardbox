use core::cmp::max;

use crate::gui::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Clone, Debug)]
pub struct UDim2 {
    pub x: UDim,
    pub y: UDim,
}

#[derive(Clone, Debug)]
pub struct UDim {
    pub scale: f32,
    pub offset: i32,
    resolution: u32,
}

impl UDim {
    pub fn as_position(&self) -> i32 {
        (self.resolution as f32 * self.scale) as i32 + self.offset
    }

    pub fn as_size(&self) -> u32 {
        max(
            (self.resolution as f32 * self.scale) as i32 + self.offset,
            0,
        ) as u32
    }
}

impl UDim2 {
    fn new(x_scale: f32, x_offset: i32, y_scale: f32, y_offset: i32) -> Self {
        UDim2 {
            x: UDim {
                scale: x_scale,
                offset: x_offset,
                resolution: SCREEN_WIDTH,
            },
            y: UDim {
                scale: y_scale,
                offset: y_offset,
                resolution: SCREEN_HEIGHT,
            },
        }
    }
}

// Constructors for different combinations of floats/integers
impl From<(f32, i32, f32, i32)> for UDim2 {
    fn from((x_scale, x_offset, y_scale, y_offset): (f32, i32, f32, i32)) -> Self {
        UDim2::new(x_scale, x_offset, y_scale, y_offset)
    }
}
impl From<(i32, i32, i32, i32)> for UDim2 {
    fn from((x_scale, x_offset, y_scale, y_offset): (i32, i32, i32, i32)) -> Self {
        UDim2::new(x_scale as f32, x_offset, y_scale as f32, y_offset)
    }
}

impl From<(f32, i32, i32, i32)> for UDim2 {
    fn from((x_scale, x_offset, y_scale, y_offset): (f32, i32, i32, i32)) -> Self {
        UDim2::new(x_scale, x_offset, y_scale as f32, y_offset)
    }
}

impl From<(i32, i32, f32, i32)> for UDim2 {
    fn from((x_scale, x_offset, y_scale, y_offset): (i32, i32, f32, i32)) -> Self {
        UDim2::new(x_scale as f32, x_offset, y_scale, y_offset)
    }
}

impl From<(i32, i32)> for UDim2 {
    fn from((x_offset, y_offset): (i32, i32)) -> Self {
        UDim2::new(0.0, x_offset, 0.0, y_offset)
    }
}
