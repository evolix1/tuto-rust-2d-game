use sdl2::rect::Rect;

use crate::positionning::{LogicalPos, RotateAngle, FlipAxis};
use crate::moves::MovePossibility;
use crate::board::EditableBoard;
use crate::game::World;

use super::error::*;
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


    pub fn render(&mut self, board: &EditableBoard, world: &World) -> Result<()> {
        self.prepare(board)?;
        self.render_all(board, world)?;
        self.draw_ctx.canvas.present();
        Ok(())
    }


    fn prepare(&mut self, board: &EditableBoard) -> Result<()> {
        self.draw_ctx.canvas.set_draw_color(self.settings.background_color);
        self.draw_ctx.canvas.clear();

        // Initialise the first time only - or when board changes
        if !self.draw_ctx.tm.borrow().sprite_exists(&SpriteId::CurrentBoard) {
            self.init_board(board)?;
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
    pub fn render_all(&mut self, board: &EditableBoard, world: &World) -> Result<()>
    {
        let (width, height) = self.draw_ctx.canvas.output_size()?;
        let geom = Rect::new(10, 10, width - 20, height - 20);

        // First, draw background
        let board_rect = self.paint_sprite(
            &SpriteId::CurrentBoard,
            geom,
            AspectRatio::KeepIn)?;

        let side = board.side_length().0;
        let side_f = side as f32;

        // Then, draw robots
        for robot in world.robots.iter() {
            let pos = match robot.pos.as_ref() {
                Some(p) => p,
                None => continue
            };

            let x = pos.x * board_rect.width() as f32 / side_f;
            let y = pos.y * board_rect.height() as f32 / side_f;

            let screen_rect = Rect::new(
                board_rect.x() + x.floor() as i32,
                board_rect.y() + y.floor() as i32,
                (board_rect.width() as f32 / side_f).floor() as u32,
                (board_rect.height() as f32 / side_f).floor() as u32,
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
        ) -> Result<Rect> {
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


    fn init_board(&mut self, board: &EditableBoard) -> Result<()> {
        let (format, width, height);

        {
            let side = board.side_length().0 as u32;
            let tm = self.draw_ctx.tm.borrow();
            let board_cell = tm.get_sprite(&SpriteId::CellBackground)?;
            format = tm.get_texture(board_cell)?.query().format;
            width = board_cell.geom.width() * side;
            height = board_cell.geom.height() * side;
        }

        let draw_walls_on_edge = self.settings.draw_walls_on_edge;

        self.draw_ctx.create_texture(
            SpriteId::CurrentBoard,
            format, width, height,
            |ctx| {
                Self::draw_board(ctx, board, draw_walls_on_edge)
            })?;

        Ok(())
    }


    fn draw_board<'c, 't>(
        draw_ctx: &mut DrawContext<'c, 't>,
        board: &EditableBoard,
        draw_walls_on_edge: bool
        ) -> Result<()>
        {
            let side = board.side_length().0;
            let side_f = side as f32;

            let (width, height) = draw_ctx.canvas
                .output_size()
                .into_sdl_error()?;

            let width = width as f32;
            let height = height as f32;

            for y in 0..side {
                for x in 0..side {
                    let px = ((x as f32 / side_f) * width).floor();
                    let py = ((y as f32 / side_f) * height).floor();

                    let next_x = (((x as f32 + 1f32) / side_f) * width).floor();
                    let next_y = (((y as f32 + 1f32) / side_f) * height).floor();

                    let geom = Rect::new(
                        px as i32,
                        py as i32,
                        (next_x - px) as u32,
                        (next_y - py) as u32);

                    let mut moves = board
                        .moves_from(&LogicalPos{ x, y })
                        .unwrap_or_else(|_| MovePossibility::all());

                    if moves.forbidden {
                        draw_ctx.draw(&SpriteId::ForbiddenCell, geom)?;
                        continue;
                    }

                    // If we don't have to paint walls next to an edge,
                    // This will simply re-enable moves, like if the user could
                    // move through it.
                    if !draw_walls_on_edge {
                        moves.up |= y == 0;
                        moves.down |= y + 1 == side;
                        moves.left |= x == 0;
                        moves.right |= x + 1 == side;
                    }


                    let flip = 
                        if (x + y) % 2 == 0 { FlipAxis::NoFlip }
                        else { FlipAxis::FlipHorizontal };


                    // base (background)
                    draw_ctx.draw_transform(
                        &SpriteId::CellBackground, geom, 
                        RotateAngle::NoTurn, flip)?;


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
