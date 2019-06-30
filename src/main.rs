use std::time::Duration;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{self, InitFlag};

// Application related
mod config;

// Math & Abstract
#[macro_use]
mod positionning;
mod moves;
mod wall;

// Game & Entities related
mod board;
mod game;

// Draw related
mod graphics;


fn main() -> Result<(), String> {
    let config = Rc::new(config::load_default()?);

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

    let draw_ctx = graphics::DrawContext::new(&mut canvas, &creator);
    draw_ctx.tm.borrow_mut().load_static(&config.assets_path)?;

    let mut renderer = graphics::Renderer::new(draw_ctx);

    let mut game = game::Game::new();

    let board_builder = board::Builder::new(&config);
    board_builder.build_on(&mut game.world);
    game.world.reset_rand_pos();

    let mut kb_controller = game::KeyboardController::new();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::B), .. } => {
                    renderer.invalidate_board();
                    board_builder.build_on(&mut game.world);
                    game.reset_rand_pos();
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    game.reset_rand_pos();
                },
                Event::KeyDown { keycode: Some(Keycode::PageUp), .. } => {
                    if !game.undo()? {
                        println!("No more action to undo.");
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::PageDown), .. } => {
                    if !game.redo()? {
                        println!("No more action to redo.");
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Home), repeat: false, .. } => {
                    while game.undo()? {
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::End), repeat: false, .. } => {
                    while game.redo()? {
                    }
                }
                _ => {
                    kb_controller.process_event(&mut game, &event)?;
                },
            }
        }

        renderer.render(&game.world)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
