use std::collections::HashMap;
use std::path::PathBuf;

use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::image::LoadSurface;
use sdl2::pixels::PixelFormatEnum;

use crate::robot::RobotId;

use super::sprite::{Sprite, SpriteId};


pub struct TextureManager<'t> {
    creator: &'t TextureCreator<WindowContext>,
    surfaces: Vec<Surface<'t>>,
    textures: Vec<Texture<'t>>,
    sprites: HashMap<SpriteId, Sprite>,
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

    pub fn load_static(&mut self, path: &PathBuf) -> Result<(), String> {
        self.surfaces = vec![
            Surface::from_file(path)?,
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
            SpriteId::SideWall,
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
