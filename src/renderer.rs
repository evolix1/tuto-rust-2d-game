use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture};

use crate::board::Board;
use crate::texture::DrawContext;


pub struct Renderer<'a> {
    background_color: Color,
    context: DrawContext<'a>,
    board_texture: Option<(usize, u32, u32)>
}


impl<'a> Renderer<'a> {
    pub fn new(context: DrawContext<'a>) -> Renderer<'a> {
        Renderer {
            background_color: Color::RGB(220, 10, 10),
            context,
            board_texture: None
        }
    }

    pub fn render(&mut self, board: &Board) -> Result<(), String> 
    {
        self.prepare()?;

        self.render_board(board)?;

        self.context.canvas.present();

        Ok(())
    }

    pub fn prepare(&mut self) -> Result<(), String> {
        self.context.canvas.set_draw_color(self.background_color);
        self.context.canvas.clear();
        
        Ok(())
    }

    pub fn render_board(
        &mut self,
        board: &Board,
        ) -> Result<(), String> 
    {
        let (width, height) = self.context.canvas.output_size()?;
        let board_screen_rect = Rect::new(10, 10, width - 20, height - 20);
        
        // If first time - we don't have generated the picture
        if self.board_texture.is_none() {
            println!("Generating board background...");
            
            // Create the texture
            let format = self.context.textures[0].query().format;
            let width = 32 * board.columns as u32;
            let height = 32 * board.rows as u32;
            
            let mut board_texture = match self.context.creator.create_texture_target(
                format,
                width,
                height) {
                Err(err) => return Err(format!("{:?}", err)),
                Ok(v) => v
            };

            // Render the texture
            let canvas = &mut self.context.canvas;
            let tile_texture = &self.context.textures[0];
            
            match canvas.with_texture_canvas(
                &mut board_texture,
                |texture_canvas| {
                    // TODO how to propagate error 
                    // (because here we cannot return something)
                    let _r = Self::paint_board(
                        texture_canvas, 
                        &tile_texture,
                        board);
                }) {
                Err(err) => return Err(format!("{:?}", err)),
                _ => ()
            }
            
            let texture_idx = self.context.add_texture(board_texture);
            self.board_texture = Some((texture_idx, width, height));
        }
        
        if let Some((id, width, height)) = self.board_texture {
            let texture = &self.context.textures[id];
            let geom = Rect::new(0, 0, width, height);
            
            self.context.canvas.copy(
                texture,
                geom, 
                board_screen_rect)?;
        }

        Ok(())
    }

    
    fn paint_board<T>(
        canvas: &mut Canvas<T>,
        texture: &Texture,
        board: &Board,
        ) -> Result<(), String> 
        where T: sdl2::render::RenderTarget
    {
        let (width, height) = canvas.output_size()?;
        let width = width as f32;
        let height = height as f32;

        let tile_geom = Rect::new(0, 0, 32, 32);

        for y in 0..board.rows {
            for x in 0..board.columns {
                let px = ((x as f32 / board.columns as f32) * width).floor();
                let py = ((y as f32 / board.rows as f32) * height).floor();

                let next_x = (((x as f32 + 1f32) / board.columns as f32) * width).floor();
                let next_y = (((y as f32 + 1f32) / board.rows as f32) * height).floor();

                let tile_screen = Rect::new(
                    px as i32, 
                    py as i32, 
                    (next_x - px) as u32, 
                    (next_y - py) as u32);
                
                canvas.copy(
                    texture, 
                    tile_geom, 
                    tile_screen)?;
            }
        }

        Ok(())
    }

}
