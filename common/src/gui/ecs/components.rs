use sdl2::pixels::Color;

use crate::gui::datatypes::UDim2;

#[derive(Clone, Debug, Default)]
pub enum Component {
    #[default]
    None,
    Position(UDim2),
    Size(UDim2),

    BackgroundColor(Color),
}

impl Component {
    pub fn identifier(&self) -> &'static str {
        match self {
            Component::None => "none",
            Component::Position(_) => "position",
            Component::Size(_) => "size",
            Component::BackgroundColor(_) => "background_color",
        }
    }
}
