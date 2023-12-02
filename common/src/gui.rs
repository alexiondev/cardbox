pub mod geometry;
pub mod util;
pub mod components;

mod traits;
pub use traits::*;

pub const WINDOW_TITLE: &str = "Cardbox";
pub const SCREEN_WIDTH: u32 = 640;
pub const SCREEN_HEIGHT: u32 = 480;