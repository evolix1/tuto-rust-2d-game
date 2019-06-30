use std::collections::HashMap;
use std::path::PathBuf;

use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::image::LoadSurface;
use sdl2::pixels::PixelFormatEnum;

use crate::game::RobotId;

use super::error::*;
use super::sprite::{Sprite, SpriteId};


pub struct TextureManager<'t> {
    creator: &'t TextureCreator<WindowContext>,
    surfaces: Vec<Surface<'t>>,
    textures: Vec<Option<Texture<'t>>>,
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

    pub fn load_static(&mut self, path: &PathBuf) -> Result<()> {
        self.surfaces = vec![
            Surface::from_file(path).into_sdl_error()?
        ];

        let texture = self.creator
            .create_texture_from_surface(&self.surfaces[0])
            .map_err(|e| format!("{:?}", e))
            .into_sdl_error()?;

        let side = texture.query().height;

        self.textures = vec![ Some(texture) ];

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
        self.textures.push(Some(texture));
        self.textures.len() - 1
    }

    pub fn get_texture(&self, sprite: &Sprite) -> Result<&Texture<'t>> {
        match self.textures.get(sprite.texture_index) {
            Some(Some(ref texture)) => Ok(texture),
            _ => bail!(ErrorKind::MissingTexture(sprite.texture_index.clone()))
        }
    }

    pub fn create_texture<F>(&mut self, format: F, width: u32, height: u32)
        -> Result<Texture<'t>>
        where F: Into<Option<PixelFormatEnum>>
    {
        self.creator
            .create_texture_target(format, width, height)
            .into_sdl_error()
    }

    // Sprite management below

    pub fn get_sprite(&self, id: &SpriteId) -> Result<&Sprite> {
        self.sprites.get(id)
            .ok_or_else(|| ErrorKind::MissingSprite(id.clone()).into())
    }

    pub fn sprite_exists(&self, id: &SpriteId) -> bool {
        self.sprites.contains_key(id)
    }

    pub fn remove_sprite(&mut self, id: &SpriteId) {
        self.sprites.remove(id)
            .and_then(|sprite| self.textures.get_mut(sprite.texture_index))
            .map(|texture| *texture = None);
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
