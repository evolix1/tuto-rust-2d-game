use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture};

use crate::board::Board;


pub fn render(
    canvas: &mut WindowCanvas,
    textures: &[Texture],
    board: &Board,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();

    let (_width, _height) = canvas.output_size()?;

    let tile_geom = Rect::new(0, 0, 32, 32);
    for y in 0i32..board.rows as i32 {
        for x in 0i32..board.columns as i32 {
            let tile_screen = Rect::new(x * 32, y * 32, 32, 32);
            canvas.copy(&textures[0], tile_geom, tile_screen)?;
        }
    }

    canvas.present();

    Ok(())
}
