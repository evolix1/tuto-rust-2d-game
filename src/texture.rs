use sdl2::render::{Texture, Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::image::{LoadTexture};


pub struct DrawContext<'a> {
    pub canvas: &'a mut Canvas<Window>,
    pub creator: &'a TextureCreator<WindowContext>,
    pub textures: Vec<Texture<'a>>,
}


impl<'a> DrawContext<'a> {
    pub fn new(
        canvas: &'a mut Canvas<Window>,
        creator: &'a TextureCreator<WindowContext>,
        ) -> DrawContext<'a> {
        DrawContext {
            canvas,
            creator,
            textures: vec![]
        }
    }

    pub fn load_static(&mut self) -> Result<(), String> {
        self.textures = vec![
            self.creator.load_texture("assets/all.png")?,
        ];
        Ok(())
    }

    pub fn add_texture(&mut self, texture: Texture<'a>) -> usize {
        self.textures.push(texture);
        self.textures.len() - 1
    }
}
