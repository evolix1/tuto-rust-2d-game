use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
//use sdl2::rect::{Point, Rect};
use sdl2::image::{self, LoadTexture, InitFlag};

mod board;
mod renderer;


fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG)?;

    let window = video_subsystem.window("Ricochet robot", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    let texture_creator = canvas.texture_creator();

    let textures = [
        texture_creator.load_texture("assets/all.png")?
    ];

    let board = board::Board::new();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        renderer::render(&mut canvas, &textures, &board)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
