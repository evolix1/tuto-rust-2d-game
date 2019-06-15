use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{self, InitFlag};

// Application related
mod config;

// Math & Abstract
#[macro_use]
mod positionning;
mod moves;
mod dim;
mod wall;

// Game & Entities related
mod board;
mod world;
mod robot;
mod keyboard_controller;

// Draw related
mod texture;
mod renderer;


fn main() -> Result<(), String> {
    let config = config::load_default()?;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG)?;

    let window = video_subsystem
        .window("Ricochet robot",
                config.window.width as u32,
                config.window.height as u32)
        .position_centered()
        .resizable()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("could not make a canvas");
    let creator = canvas.texture_creator();

    let draw_ctx = texture::DrawContext::new(&mut canvas, &creator);
    draw_ctx.tm.borrow_mut().load_static(&config.assets_path)?;

    let mut renderer = renderer::Renderer::new(draw_ctx);

    let mut world = world::GameWorld::new(&config);
    world.reset_rand_pos();

    let mut kb_controller = keyboard_controller::KeyboardController::new();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::R), repeat: false, .. } => {
                    world.reset_rand_pos();
                },
                _ => {
                    kb_controller.process_event(&mut world, &event)?;
                },
            }
        }

        renderer.render(&world)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
