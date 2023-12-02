use crate::gui::{Drawable, geometry::UDim2};

pub trait GuiObject {
    fn position(&self) -> UDim2;
    fn size(&self) -> UDim2;
}