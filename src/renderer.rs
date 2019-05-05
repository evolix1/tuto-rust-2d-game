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
    
    let (width, height) = canvas.output_size()?;
    let board_screen_rect = Rect::new(10, 10, width - 20, height - 20);
    render_board(canvas, &textures[0], board, &board_screen_rect)?;

    canvas.present();

    Ok(())
}


fn render_board(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    board: &Board,
    rect: &Rect
) -> Result<(), String> {
    let width = rect.width() as f32;
    let height = rect.height() as f32;

    let tile_geom = Rect::new(0, 0, 32, 32);
    
    for y in 0..board.rows {
        for x in 0..board.columns {
            let px = ((x as f32 / board.columns as f32) * width).floor();
            let py = ((y as f32 / board.rows as f32) * height).floor();
            
            let next_x = (((x as f32 + 1f32) / board.columns as f32) * width).floor();
            let next_y = (((y as f32 + 1f32) / board.rows as f32) * height).floor();
            
            let tile_screen = Rect::new(
                rect.x() + px as i32, 
                rect.y() + py as i32, 
                (next_x - px) as u32, 
                (next_y - py) as u32);
            canvas.copy(&texture, tile_geom, tile_screen)?;
        }
    }

    Ok(())
}
