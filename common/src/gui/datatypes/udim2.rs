#[derive(Debug)]
pub struct UDim2 {
    pub x: UDim,
    pub y: UDim,
}

#[derive(Debug)]
pub struct UDim {
    pub scale: f32,
    pub offset: i32,
}

impl UDim2 {
    fn new(x_scale: f32, x_offset: i32, y_scale: f32, y_offset: i32) -> Self {
        UDim2 {
            x: UDim {
                scale: x_scale,
                offset: x_offset,
            },
            y: UDim {
                scale: y_scale,
                offset: y_offset,
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
