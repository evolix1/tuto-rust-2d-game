use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::path::Path;

use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::render::{Texture, Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::image::LoadSurface;
use sdl2::pixels::PixelFormatEnum;

use crate::robot::RobotId;


pub struct DrawContext<'c, 't> {
    pub canvas: &'c mut Canvas<Window>,
    pub tm: Rc<RefCell<TextureManager<'t>>>,
}


pub struct TextureManager<'t> {
    creator: &'t TextureCreator<WindowContext>,
    surfaces: Vec<Surface<'t>>,
    textures: Vec<Texture<'t>>,
    sprites: HashMap<SpriteId, Sprite>,
}


#[derive(PartialEq, Eq, Hash)]
pub enum SpriteId {
    // Board management
    CellBackground,
    SizedBoard { width: u32, height: u32 },
    DefaultBoard,
    // Corner overlay
    CornerWall,
    SideWall(u8),
    //
    Robot(RobotId),
}


#[derive(Clone)]
pub struct Sprite {
    pub texture_index: usize,
    pub geom: Rect
}
        

#[allow(dead_code)]
pub enum FlipAxis {
    NoFlip,
    FlipHorizontal,
    FlipVertical,
    FlipBoth
}

#[allow(dead_code)]
pub enum RotateAngle {
    NoTurn,
    TurnLeft,
    TurnRight,
    HalfTurn
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


impl<'t> TextureManager<'t> {
    pub fn new(
        creator: &'t TextureCreator<WindowContext>,
        ) -> TextureManager<'t> {
        TextureManager {
            creator,
            surfaces: Vec::new(),
            textures: Vec::new(),
            sprites: HashMap::new(),
        }
    }

    pub fn load_static(&mut self) -> Result<(), String> {
        self.surfaces = vec![
            Surface::from_file(&Path::new("assets/all.svg"))?,
        ];
        
        self.textures = vec![
            self.creator.create_texture_from_surface(&self.surfaces[0])
                .map_err(|e| format!("{:?}", e))?,
        ];

        let side = self.textures[0].query().height;

        let ids = vec![
            SpriteId::Robot(RobotId::Red),
            SpriteId::Robot(RobotId::Green),
            SpriteId::Robot(RobotId::Blue),
            SpriteId::Robot(RobotId::Yellow),
            SpriteId::CellBackground,
            SpriteId::CornerWall,
            SpriteId::SideWall(1),
            SpriteId::SideWall(2),
            SpriteId::SideWall(3),
        ];
        
        for (i, id) in ids.into_iter().enumerate() {
            let sprite = Sprite { 
                texture_index: 0, 
                geom: Rect::new(i as i32 * side as i32, 0, side, side) 
            };
            self.sprites.insert(id, sprite);
        }
        
        Ok(())
    }

    // Texture management below
    
    pub fn add_texture(&mut self, texture: Texture<'t>) -> usize {
        self.textures.push(texture);
        self.textures.len() - 1
    }

    pub fn get_texture(&self, sprite: &Sprite) -> Result<&Texture<'t>, String> {
        self.textures.get(sprite.texture_index)
            .ok_or_else(|| format!("missing texture"))
    }

    pub fn create_texture<F>(&mut self, format: F, width: u32, height: u32) 
        -> Result<Texture<'t>, String> 
        where F: Into<Option<PixelFormatEnum>>
    {
        self.creator
            .create_texture_target(format, width, height)
            .map_err(|err| format!("{:?}", err))
    }

    // Sprite management below
    
    pub fn get_sprite(&self, id: &SpriteId) -> Result<&Sprite, String> {
        self.sprites.get(id)
            .ok_or_else(|| format!("missing sprite"))
    }

    pub fn sprite_exists(&self, id: &SpriteId) -> bool {
        self.sprites.contains_key(id)
    }
    
    pub fn set_sprite(&mut self, id: SpriteId, sprite: Sprite) {
        self.sprites.insert(id, sprite);
    }
    
    pub fn add_sprite_from_texture(&mut self, texture: Texture<'t>, id: SpriteId) -> Sprite {
        let info = texture.query();
        let geom = Rect::new(0, 0, info.width, info.height);
        
        let texture_index = self.add_texture(texture);
        
        let sprite = Sprite { texture_index, geom };
        self.sprites.insert(id, sprite.clone());
        
        sprite
    }

}
