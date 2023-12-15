use std::{fs, thread::sleep, time::Duration};

use sdl2::{pixels::Color, rect::Rect};

use common::{
    data::save::Sav1,
    gui::{
        datatypes::UDim2,
        ecs::{Component::*, Screen},
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

    let mut screen = Screen::new();
    let e1 = screen
        .new_entity()
        .with(Position(UDim2::from((0, 0))))
        .with(Size(UDim2::from((1, 1))))
        .build();
    let e2 = screen
        .new_entity()
        .with(Position(UDim2::from((0.5, 0, 0.5, 0))))
        .with(Size(UDim2::from((0.1, 0, 0.1, 0))))
        .build();

    println!("{:?}", screen);
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    for id in 0..screen.entity_count {
        let pos: &UDim2;
        let size: &UDim2;
        match &screen.components["position"][id] {
            Position(p) => pos = &p,
            _ => todo!(),
        }

        match &screen.components["size"][id] {
            Size(s) => size = &s,
            _ => todo!(),
        }

        canvas
            .fill_rect(Rect::new(
                pos.x.as_position(),
                pos.y.as_position(),
                size.x.as_size(),
                size.y.as_size(),
            ))
            .unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 0));
    }

    canvas.present();
    sleep(Duration::new(2, 0));

    Ok(())
}
