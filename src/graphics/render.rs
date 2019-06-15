use sdl2::pixels::Color;


#[allow(dead_code)]
pub enum AspectRatio {
    /// Dimensions are scaled to fit the container, ratio is not preserved
    Stretch,    
    /// Preserve ratio, and make it fit inside the container
    KeepIn,     
    // NOTE: missing 3rd options, that overflow its container
}


pub struct RenderSettings {
    pub background_color: Color,
    pub draw_walls_on_edge: bool,
}


impl RenderSettings {
    pub fn new() -> RenderSettings {
        RenderSettings {
            background_color: Color::RGB(220, 10, 10),
            draw_walls_on_edge: false,
        }
    }
}
