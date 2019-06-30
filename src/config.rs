use std::path::{Path, PathBuf, Component};
use std::fs::File;
use std::io::Read;
use std::collections::VecDeque;

use serde_derive::Deserialize;

use crate::positionning::SideLength;
use crate::board;

use super::error::*;


#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub window: WindowConfig,
    pub assets_path: PathBuf,
    #[serde(rename = "side_length")]
    pub board_side_length: SideLength,
    #[serde(rename = "tiles")]
    pub tile_sets: Vec<board::TileSet>,
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
pub fn load_default() -> Result<AppConfig> {
    load(Path::new("./config.json5"))
}


pub fn load(path: &Path) -> Result<AppConfig> {
    let path_solver = PathSolver::new()?;

    let mut config: AppConfig = File::open(path)
        .map_err(Error::from)
        .and_then(|mut file| {
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map(|_| content)
                .map_err(Error::from)
        })
        .and_then(|content| json5::from_str(content.as_str()).map_err(Error::from))
        .and_then(|mut option: AppConfig| {
            let assets_path = std::mem::replace(&mut option.assets_path, PathBuf::new());
            option.assets_path = path_solver.resolve(assets_path);
            Ok(option)
        })?;


    // manually load tiles
    for tile_set in config.tile_sets.iter_mut() {
        tile_set.parse()?;
    }

    Ok(config)
}


struct PathSolver(PathBuf);


impl PathSolver {
    pub fn new() -> Result<Self> {
        let exe_path = std::env::current_exe()?;
        Ok(PathSolver(exe_path))
    }


    pub fn resolve<T>(&self, unknown_path: T) -> PathBuf where T: Into<PathBuf> {
        let unknown_path = unknown_path.into();

        let full_path =
            if unknown_path.is_relative() { self.0.join(&unknown_path) }
            else { unknown_path };

        self.make_absolute(full_path)
    }


    pub fn make_absolute(&self, path: PathBuf) -> PathBuf {
        let mut queue = VecDeque::new();

        for component in path.components() {
            match component {
                Component::ParentDir => { queue.pop_back(); },
                _ => queue.push_back(component),
            }
        }

        queue.into_iter()
            .fold(PathBuf::new(), |acc, comp| {
                acc.join(comp)
            })
    }
}
