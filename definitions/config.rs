use typedef::*;
use value::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisplayConfig {
    pub resolution: DisplayResolution,
    pub scale: Float,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        DisplayConfig {
            resolution: Default::default(),
            scale: 4.0,
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
pub struct VMConfig {
    pub title: String,
    pub display: DisplayConfig,
    pub keyboard_enabled: bool,
    pub mouse_enabled: bool,
}

impl Default for VMConfig {
    fn default() -> Self {
        VMConfig {
            title: "bakerVM".into(),
            display: Default::default(),
            keyboard_enabled: true,
            mouse_enabled: true,
        }
    }
}
