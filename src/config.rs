use std::path::Path;
use std::fs::File;
use std::io::Read;

use serde_derive::Deserialize;


#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub window: WindowConfig,
    pub assets_path: String,
}


#[derive(Debug, Deserialize)]
pub struct WindowConfig {
    #[serde(default = "defaults::width")]
    pub width: usize,
    #[serde(default = "defaults::height")]
    pub height: usize,
}


impl Default for WindowConfig {
    fn default() -> WindowConfig {
        WindowConfig {
            width: 800,
            height: 800,
        }
    }
}


mod defaults {
    use super::WindowConfig;

    pub fn width() -> usize { WindowConfig::default().width }
    pub fn height() -> usize { WindowConfig::default().height }
}


// loaders
pub fn load_default() -> Result<AppConfig, String> {
    load(Path::new("./config.json5"))
}


pub fn load(path: &Path) -> Result<AppConfig, String> {
    File::open(path)
        .and_then(|mut file| {
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map(|_| content)
        })
        // NOTE: io::Error to string because, string is enough for now
        .map_err(|e| format!("{:?}", e))
        // parse the file
        .and_then(
            |content| json5::from_str(content.as_str())
            // NOTE: horrendous error handling, but that what the crate gives us
            // so...
            .map_err(|e| match e {
                json5::Error::Message(m) => m
            })
        )
}
