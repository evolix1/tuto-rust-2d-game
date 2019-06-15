use std::cell::RefCell;
use std::rc::Rc;

use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::pixels::PixelFormatEnum;

use crate::positionning::{RotateAngle, FlipAxis};

use super::sprite::{Sprite, SpriteId};
use super::texture::TextureManager;


pub struct DrawContext<'c, 't> {
    pub canvas: &'c mut Canvas<Window>,
    pub tm: Rc<RefCell<TextureManager<'t>>>,
}


impl<'c, 't> DrawContext<'c, 't> {
    pub fn new(
        canvas: &'c mut Canvas<Window>,
        creator: &'c TextureCreator<WindowContext>,
        ) -> DrawContext<'c, 'c> {
        DrawContext {
            canvas,
            tm: Rc::new(RefCell::new(TextureManager::new(creator))),
        }
    }

    pub fn draw(&mut self, id: &SpriteId, area: Rect) -> Result<(), String> {
        let tm = self.tm.borrow();
        let sprite = tm.get_sprite(id)?;
        let texture = tm.get_texture(sprite)?;
        self.canvas.copy(texture, sprite.geom, area)
    }

    pub fn draw_transform(
        &mut self,
        id: &SpriteId,
        area: Rect,
        rotation: RotateAngle,
        flip: FlipAxis,
        ) -> Result<(), String> {
        let angle = match rotation {
            RotateAngle::NoTurn => 0f64,
            RotateAngle::TurnLeft => 90f64,
            RotateAngle::TurnRight => 270f64,
            RotateAngle::HalfTurn => 180f64,
        };
        let (flip_horizontal, flip_vertical) = match flip {
            FlipAxis::NoFlip => (false, false),
            FlipAxis::FlipHorizontal => (true, false),
            FlipAxis::FlipVertical => (false, true),
            FlipAxis::FlipBoth => (true, true),
        };
        let center = None;

        let tm = self.tm.borrow();
        let sprite = tm.get_sprite(id)?;
        let texture = tm.get_texture(sprite)?;
        self.canvas.copy_ex(texture, sprite.geom, area,
                            angle, center, flip_horizontal, flip_vertical)
    }

    pub fn create_texture<F, D>(
        &mut self,
        id: SpriteId,
        format: F,
        width: u32,
        height: u32,
        draw: D
        )
        -> Result<Sprite, String>
        where F: Into<Option<PixelFormatEnum>>,
              D: for<'m> FnOnce(&'m mut DrawContext<'m, 't>) -> Result<(), String>,
    {
        let mut texture = self.tm.borrow_mut().create_texture(format, width, height)?;

        let mut draw_result = Ok(());
        let reuse_tm = self.tm.clone();

        self.canvas.with_texture_canvas(
            &mut texture,
            |texture_canvas| {
                let mut ctx = DrawContext { canvas: texture_canvas, tm: reuse_tm };
                draw_result = draw(&mut ctx);
            })
            .map_err(|err| format!("{:?}", err))
            .and(draw_result)
            .map(|_| self.tm.borrow_mut().add_sprite_from_texture(texture, id))
    }

}
