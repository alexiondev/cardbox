
use std::{fs, thread::sleep, time::Duration};

use sdl2::{pixels::Color, rect::Rect};

use common::{data::{save::Sav1, Species}, gui::{util::sdl_init, geometry::UDim2, components::{Frame, Image}, Drawable}};

fn main() -> anyhow::Result<()> {
    let data = fs::read("blue.srm")?;
    let sav = Sav1::from(data);

    let (context, mut canvas) = sdl_init();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    let pos = UDim2::new(0.0, 10, 0.0, 10);
    let size = UDim2::new(1.0, -20, 1.0, -20);

    let frame = Frame::new(pos, size);
    frame.draw(&mut canvas);

    for i in 0..sav.curr_box.len() {
        match sav.curr_box[i].species {
            Species::DEFAULT | Species::UNKNOWN => continue,
            _ => {
                let pos = UDim2::new(0.2*(i%5) as f32, 0, 0.25*(i/5) as f32, 0);
                let size = UDim2::new(0.0, 68*2, 0.0, 56*2);
                let file = format!("assets/sprites/box/{}.png", sav.curr_box[i].species.to_filename());
                let img = Image::new(pos, size, &file);
                img.draw(&mut canvas);
            }
        }
    }
    // let size = UDim2::new(0.0, 68*2, 0.0, 56*2);
    // let img = Image::new(pos, size, "assets/sprites/box/bulbasaur.png");
    // img.draw(&mut canvas);

    canvas.present();

    sleep(Duration::new(10,0));

    Ok(())
}
