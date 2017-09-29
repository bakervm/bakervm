//! The configuration format for the program container

use typedef::*;

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
            default_scale: 4.0,
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
            width: 160,
            height: 100,
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
            title: "bakerVM".into(),
            display: Default::default(),
            input_enabled: true,
        }
    }
}
