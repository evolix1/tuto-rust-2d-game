use std::path::Path;
use std::collections::HashMap;

use sdl2::rect::{Rect};
use sdl2::surface::{Surface};
use sdl2::render::{Texture, Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::image::{LoadSurface};
use sdl2::pixels::{PixelFormatEnum};

use crate::robot::RobotId;


pub struct DrawContext<'a> {
    pub canvas: &'a mut Canvas<Window>,
    creator: &'a TextureCreator<WindowContext>,
    pub surfaces: Vec<Surface<'a>>,
    pub textures: Vec<Texture<'a>>,
    pub sprites: HashMap<SpriteId, Sprite>,
}


#[derive(PartialEq, Eq, Hash)]
pub enum SpriteId {
    // Board management
    BoardCell,
    SizedBoard { width: u32, height: u32 },
    DefaultBoard,
    //
    Robot(RobotId),
}


#[derive(Clone)]
pub struct Sprite {
    pub texture_index: usize,
    pub geom: Rect
}


impl<'a> DrawContext<'a> {
    pub fn new(
        canvas: &'a mut Canvas<Window>,
        creator: &'a TextureCreator<WindowContext>,
        ) -> DrawContext<'a> {
        DrawContext {
            canvas,
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
        
        self.sprites.insert(
            SpriteId::BoardCell,
            Sprite { texture_index: 0, geom: Rect::new(0, 0, side, side) });
        
        // Robots
        self.sprites.insert(
            SpriteId::Robot(RobotId::Blue),
            Sprite { texture_index: 0, geom: Rect::new(1 * side as i32, 0, side, side) });
        self.sprites.insert(
            SpriteId::Robot(RobotId::Green),
            Sprite { texture_index: 0, geom: Rect::new(2 * side as i32, 0, side, side) });
        self.sprites.insert(
            SpriteId::Robot(RobotId::Yellow),
            Sprite { texture_index: 0, geom: Rect::new(3 * side as i32, 0, side, side) });
        self.sprites.insert(
            SpriteId::Robot(RobotId::Red),
            Sprite { texture_index: 0, geom: Rect::new(4 * side as i32, 0, side, side) });
        
        Ok(())
    }

    pub fn create_texture<F>(&self, format: F, width: u32, height: u32) 
        -> Result<Texture<'a>, String> 
        where F: Into<Option<PixelFormatEnum>>
    {
        self.creator
            .create_texture_target(format, width, height)
            .map_err(|err| format!("{:?}", err))
    }

    pub fn add_sprite_from_texture(&mut self, texture: Texture<'a>, id: SpriteId) -> Sprite {
        let info = texture.query();
        let geom = Rect::new(0, 0, info.width, info.height);
        
        let texture_index = self.add_texture(texture);
        
        let sprite = Sprite { texture_index, geom };
        self.sprites.insert(id, sprite.clone());
        
        sprite
    }

    pub fn add_texture(&mut self, texture: Texture<'a>) -> usize {
        self.textures.push(texture);
        self.textures.len() - 1
    }

    pub fn sprite_exists(&self, id: &SpriteId) -> bool {
        self.sprites.contains_key(id)
    }
}
