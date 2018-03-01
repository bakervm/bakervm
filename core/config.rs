//! The configuration format for the program container

use typedef::*;

pub const DEFAULT_SCALE: f64 = 4.0;

pub const DEFAULT_SCREEN_WIDTH: usize = 160;
pub const DEFAULT_SCREEN_HEIGHT: usize = 100;

pub const DEFAULT_WINDOW_TITLE: &str = "bakerVM";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisplayConfig {
    #[serde(default)]
    pub resolution: DisplayResolution,
    pub default_scale: Float,
    #[serde(default)]
    pub hide_cursor: bool,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        DisplayConfig {
            resolution: Default::default(),
            default_scale: DEFAULT_SCALE,
            hide_cursor: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisplayResolution {
    pub width: usize,
    pub height: usize,
}

impl Default for DisplayResolution {
    fn default() -> Self {
        DisplayResolution {
            width: DEFAULT_SCREEN_WIDTH,
            height: DEFAULT_SCREEN_HEIGHT,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub display: DisplayConfig,
    #[serde(default)]
    pub input_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            title: DEFAULT_WINDOW_TITLE.into(),
            display: Default::default(),
            input_enabled: true,
        }
    }
}
