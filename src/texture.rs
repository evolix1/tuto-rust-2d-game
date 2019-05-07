use std::collections::HashMap;

use sdl2::rect::{Rect};
use sdl2::render::{Texture, Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::image::{LoadTexture};


pub struct DrawContext<'a> {
    pub canvas: &'a mut Canvas<Window>,
    pub creator: &'a TextureCreator<WindowContext>,
    pub textures: Vec<Texture<'a>>,
    pub sprites: HashMap<SpriteId, Sprite>,
}


impl<'a> DrawContext<'a> {
    pub fn new(
        canvas: &'a mut Canvas<Window>,
        creator: &'a TextureCreator<WindowContext>,
        ) -> DrawContext<'a> {
        DrawContext {
            canvas,
            creator,
            textures: Vec::new(),
            sprites: HashMap::new(),
        }
    }

    pub fn load_static(&mut self) -> Result<(), String> {
        self.textures = vec![
            self.creator.load_texture("assets/all.png")?,
        ];
        
        self.sprites.insert(
            SpriteId::BoardCell,
            Sprite { texture_id: 0, geom: Rect::new(0, 0, 32, 32) });
        
        Ok(())
    }

    pub fn add_sprite_from_texture(&mut self, texture: Texture<'a>, id: SpriteId) -> Sprite {
        let info = texture.query();
        let geom = Rect::new(0, 0, info.width, info.height);
        
        let texture_id = self.add_texture(texture);
        
        let sprite = Sprite { texture_id, geom };
        self.sprites.insert(id, sprite.clone());
        
        sprite
    }

    pub fn add_texture(&mut self, texture: Texture<'a>) -> usize {
        self.textures.push(texture);
        self.textures.len() - 1
    }
}


#[derive(PartialEq, Eq, Hash)]
pub enum SpriteId {
    BoardCell,
    SizedBoard { width: u32, height: u32 },
    DefaultBoard,
}

#[derive(Clone)]
pub struct Sprite {
    pub texture_id: usize,
    pub geom: Rect
}
