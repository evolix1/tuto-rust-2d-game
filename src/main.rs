use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{self, InitFlag};

// Game & Entities related
mod positionning;
mod board;
mod world;
mod robot;

// Draw related
mod texture;
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
        
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("could not make a canvas");
    let creator = canvas.texture_creator();

    let draw_ctx = texture::DrawContext::new(&mut canvas, &creator);
    draw_ctx.tm.borrow_mut().load_static()?;
    
    let mut renderer = renderer::Renderer::new(draw_ctx);
    
    let mut world = world::GameWorld::new();
    world.reset_rand_pos();

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
                _ => {}
            }
        }

        renderer.render(&world)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
