use sdl2::rect::Rect;

use crate::world::GameWorld;
use crate::positionning::{Pos, RotateAngle, FlipAxis};
use crate::moves::MovePossibility;

use super::draw::DrawContext;
use super::sprite::SpriteId;
use super::render::{RenderSettings, AspectRatio};


pub struct Renderer<'r> {
    settings: RenderSettings,
    draw_ctx: DrawContext<'r, 'r>,
}


impl<'r> Renderer<'r> {
    pub fn new(draw_ctx: DrawContext<'r, 'r>) -> Renderer<'r> {
        let settings = RenderSettings::new();
        Renderer { settings, draw_ctx, }
    }

    
    pub fn render(&mut self, world: &GameWorld) -> Result<(), String> {
        self.prepare(world)?;
        self.render_all(world)?;
        self.draw_ctx.canvas.present();
        Ok(())
    }

    
    fn prepare(&mut self, world: &GameWorld) -> Result<(), String> {
        self.draw_ctx.canvas.set_draw_color(self.settings.background_color);
        self.draw_ctx.canvas.clear();

        // Initialise the first time only - or when board changes
        if !self.draw_ctx.tm.borrow().sprite_exists(&SpriteId::CurrentBoard) {
            self.init_board(world)?;
        }

        Ok(())
    }

    pub fn invalidate_board(&mut self) {
        self.draw_ctx.tm.borrow_mut()
            .remove_sprite(&SpriteId::CurrentBoard);
    }
    
    /**
     * Render all game items.
     */
    pub fn render_all(&mut self, world: &GameWorld) -> Result<(), String> 
    {
        let (width, height) = self.draw_ctx.canvas.output_size()?;
        let geom = Rect::new(10, 10, width - 20, height - 20);

        // First, draw background
        let board_rect = self.paint_sprite(
            &SpriteId::CurrentBoard, 
            geom, 
            AspectRatio::KeepIn)?;

        let dim = world.board.dim();

        // Then, draw robots
        for robot in world.robots.iter() {
            let pos = match robot.pos.as_ref() {
                Some(p) => p,
                None => continue
            };
                
            let x = pos.x as f32 * board_rect.width() as f32 / dim.columns as f32;
            let y = pos.y as f32 * board_rect.height() as f32 / dim.rows as f32;

            let screen_rect = Rect::new(
                board_rect.x() + x.floor() as i32,
                board_rect.y() + y.floor() as i32,
                (board_rect.width() as f32 / dim.columns as f32).floor() as u32,
                (board_rect.height() as f32 / dim.rows as f32).floor() as u32,
                );
        
            let _ = self.paint_sprite(
                &SpriteId::Robot(robot.id.clone()), 
                screen_rect, 
                AspectRatio::Stretch)?;
        }

        Ok(())
    }


    fn paint_sprite(
        &mut self, 
        id: &SpriteId, 
        area: Rect,
        aspect: AspectRatio
        ) -> Result<Rect, String> {
        let display_geom = match aspect {
            AspectRatio::Stretch => area,
            AspectRatio::KeepIn => {
                let tm = self.draw_ctx.tm.borrow();
                let sprite = tm.get_sprite(id)?;
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
        
        self.draw_ctx.draw(id, display_geom)?;
        
        Ok(display_geom)
    }


    fn init_board(&mut self, world: &GameWorld) -> Result<(), String> {
        let (format, width, height);
        
        {
            let dim = world.board.dim();
            let tm = self.draw_ctx.tm.borrow();
            let board_cell = tm.get_sprite(&SpriteId::CellBackground)?;
            format = tm.get_texture(board_cell)?.query().format;
            width = board_cell.geom.width() * dim.columns as u32;
            height = board_cell.geom.height() * dim.rows as u32;
        }
                
        let draw_walls_on_edge = self.settings.draw_walls_on_edge;

        self.draw_ctx.create_texture(
            SpriteId::CurrentBoard,
            format, width, height,
            |ctx| {
                Self::draw_board(ctx, world, draw_walls_on_edge)
            })?;
        
        Ok(())
    }


    fn draw_board<'c, 't>(
        draw_ctx: &mut DrawContext<'c, 't>,
        world: &GameWorld,
        draw_walls_on_edge: bool
        ) -> Result<(), String> 
        {
            let sprite_id = SpriteId::CellBackground;

            let dim = world.board.dim();
            
            let (width, height) = draw_ctx.canvas.output_size()?;
            let width = width as f32;
            let height = height as f32;

            for y in 0..dim.rows {
                for x in 0..dim.columns {
                    let px = ((x as f32 / dim.columns as f32) * width).floor();
                    let py = ((y as f32 / dim.rows as f32) * height).floor();

                    let next_x = (((x as f32 + 1f32) / dim.columns as f32) * width).floor();
                    let next_y = (((y as f32 + 1f32) / dim.rows as f32) * height).floor();

                    let geom = Rect::new(
                        px as i32, 
                        py as i32, 
                        (next_x - px) as u32, 
                        (next_y - py) as u32);
                    
                    let mut moves = world.board
                        .moves_from(&Pos::new(x, y))
                        .unwrap_or_else(|_| MovePossibility::all());

                    // If we don't have to paint walls next to an edge, 
                    // This will simply re-enable moves, like if the user could
                    // move through it.
                    if !draw_walls_on_edge {
                        moves.up |= y == 0;
                        moves.down |= y + 1 == dim.rows;
                        moves.left |= x == 0;
                        moves.right |= x + 1 == dim.columns;
                    }
                    

                    // base (background)
                    draw_ctx.draw(&sprite_id, geom)?;

                    
                    // Walls between cells
                    if !moves.left {
                        draw_ctx.draw_transform(
                            &SpriteId::SideWall, geom,
                            RotateAngle::TurnRight, FlipAxis::NoFlip)?;
                    }
                    if !moves.right {
                        draw_ctx.draw_transform(
                            &SpriteId::SideWall, geom,
                            RotateAngle::TurnLeft, FlipAxis::NoFlip)?;
                    }
                    if !moves.up {
                        draw_ctx.draw_transform(
                            &SpriteId::SideWall, geom,
                            RotateAngle::NoTurn, FlipAxis::NoFlip)?;
                    }
                    if !moves.down {
                        draw_ctx.draw_transform(
                            &SpriteId::SideWall, geom,
                            RotateAngle::HalfTurn, FlipAxis::NoFlip)?;
                    }

                    
                    // Corners
                    let top_left = !moves.up || !moves.left;
                    let top_right = !moves.up || !moves.right;
                    let down_left = !moves.down || !moves.left;
                    let down_right = !moves.down || !moves.right;
                    
                    if top_left {
                        draw_ctx.draw_transform(
                            &SpriteId::CornerWall, geom,
                            RotateAngle::NoTurn, FlipAxis::NoFlip)?;
                    }
                    if top_right {
                        draw_ctx.draw_transform(
                            &SpriteId::CornerWall, geom,
                            RotateAngle::TurnLeft, FlipAxis::NoFlip)?;
                    }
                    if down_left {
                        draw_ctx.draw_transform(
                            &SpriteId::CornerWall, geom,
                            RotateAngle::TurnRight, FlipAxis::NoFlip)?;
                    }
                    if down_right {
                        draw_ctx.draw_transform(
                            &SpriteId::CornerWall, geom,
                            RotateAngle::HalfTurn, FlipAxis::NoFlip)?;
                    }
                    
                }
            }

            Ok(())
        }

}
