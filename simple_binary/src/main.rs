use std::{fs, thread::sleep, time::Duration};

use sdl2::pixels::Color;

use common::{
    data::save::Sav1,
    gui::{
        datatypes::UDim2,
        util::sdl_init,
    },
};

fn main() -> anyhow::Result<()> {
    let data = fs::read("blue.srm")?;
    let sav = Sav1::from(data);

    let (context, mut canvas) = sdl_init();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    canvas.present();

    sleep(Duration::new(2, 0));

    Ok(())
}
