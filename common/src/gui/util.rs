
use sdl2::{self, render::Canvas, video::Window, Sdl};

use super::{WINDOW_TITLE, SCREEN_WIDTH, SCREEN_HEIGHT};

pub fn sdl_init() -> (Sdl, Canvas<Window>) {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    let window = video.window(WINDOW_TITLE, SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();
    return (context, canvas)
}