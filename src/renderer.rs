use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture};

use crate::world::GameWorld;
use crate::texture::{DrawContext, SpriteId};


#[allow(dead_code)]
pub enum AspectRatio {
    /// Dimensions are scaled to fit the container
    Stretch,    
    /// Preserve ratio, and make it fit inside the container
    KeepIn,     
}


pub struct Renderer<'a> {
    background_color: Color,
    draw_ctx: DrawContext<'a>,
}


impl<'a> Renderer<'a> {
    pub fn new(draw_ctx: DrawContext<'a>) -> Renderer<'a> {
        Renderer {
            background_color: Color::RGB(220, 10, 10),
            draw_ctx,
        }
    }

    
    pub fn render(&mut self, world: &GameWorld) -> Result<(), String> 
    {
        self.prepare(world)?;

        self.render_all()?;

        self.draw_ctx.canvas.present();

        Ok(())
    }

    
    pub fn prepare(&mut self, world: &GameWorld) -> Result<(), String> {
        self.draw_ctx.canvas.set_draw_color(self.background_color);
        self.draw_ctx.canvas.clear();

        if !self.draw_ctx.sprites.contains_key(&SpriteId::DefaultBoard) {
            self.init_board(world)?;
        }

        Ok(())
    }
    
    
    /**
     * Render all game items.
     */
    pub fn render_all(&mut self) -> Result<(), String> 
    {
        let (width, height) = self.draw_ctx.canvas.output_size()?;
        let geom = Rect::new(10, 10, width - 20, height - 20);

        //let paint_rect = Self::keepAspectRatio(screen_rect, sprite.geom);
        self.paint_sprite(&SpriteId::DefaultBoard, geom, AspectRatio::KeepIn)?;

        Ok(())
    }


    // Private API below
    fn paint_sprite(
        &mut self, 
        id: &SpriteId, 
        area: Rect,
        aspect: AspectRatio
        ) -> Result<(), String> {
        let sprite = self.draw_ctx.sprites.get(id)
            .ok_or_else(|| format!("missing sprite"))?;
        
        let texture = &self.draw_ctx.textures[sprite.texture_id];
        
        let display_geom = match aspect {
            AspectRatio::Stretch => area,
            AspectRatio::KeepIn => {
                let width = (area.height() as f32 
                             * sprite.geom.width() as f32 
                             / sprite.geom.height() as f32).floor() as u32;
                let height = (area.width() as f32 
                              * sprite.geom.height() as f32 
                              / sprite.geom.width() as f32).floor() as u32;
                
                if width < area.width() {
                    Rect::from_center(area.center(), width, area.height())
                } else {
                    Rect::from_center(area.center(), area.width(), height)
                }
            },
        };
        
        self.draw_ctx.canvas.copy(texture, sprite.geom, display_geom)?;
        Ok(())
    }


    fn init_board(&mut self, world: &GameWorld) -> Result<(), String> {
        let board_cell = self.draw_ctx.sprites
            .get(&SpriteId::BoardCell)
            .expect("board cell sprite exists");

        let format = self.draw_ctx.textures[board_cell.texture_id].query().format;
        let width = board_cell.geom.width() * world.board.columns as u32;
        let height = board_cell.geom.height() * world.board.rows as u32;
        
        // Create the texture
        let mut board_texture = self.draw_ctx.creator
            .create_texture_target(format, width, height)
            .map_err(|err| format!("{:?}", err))?;

        // Render the texture
        let canvas = &mut self.draw_ctx.canvas;
        let tile_texture = &self.draw_ctx.textures[board_cell.texture_id];

        let mut draw_err = Ok(());
        canvas.with_texture_canvas(
            &mut board_texture,
            |texture_canvas| { 
                draw_err = Self::draw_board(texture_canvas, &tile_texture, world);
            })
            .map_err(|err| format!("{:?}", err))?;
        // re-raise error of draw
        draw_err?;

        // save texture
        let board = self.draw_ctx.add_sprite_from_texture(
            board_texture, 
            SpriteId::SizedBoard{ width, height });
        
        // Remember this sprite for being the default board sprite
        self.draw_ctx.sprites.insert(SpriteId::DefaultBoard, board);

        Ok(())
    }


    fn draw_board<T>(
        canvas: &mut Canvas<T>,
        texture: &Texture,
        world: &GameWorld,
        ) -> Result<(), String> 
        where T: sdl2::render::RenderTarget
        {
            let (width, height) = canvas.output_size()?;
            let width = width as f32;
            let height = height as f32;

            let tile_geom = Rect::new(0, 0, 32, 32);

            for y in 0..world.board.rows {
                for x in 0..world.board.columns {
                    let px = ((x as f32 / world.board.columns as f32) * width).floor();
                    let py = ((y as f32 / world.board.rows as f32) * height).floor();

                    let next_x = (((x as f32 + 1f32) / world.board.columns as f32) * width).floor();
                    let next_y = (((y as f32 + 1f32) / world.board.rows as f32) * height).floor();

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
