use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Debug, Resource, Serialize)]
#[serde(default)]
pub struct GameConfig {
    pub audio: AudioConfig,
    pub controls: ControlsConfig,
    pub general: GeneralConfig,
    pub visual: VisualConfig,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct AudioConfig {
    pub master_volume: f32,
    pub music_volume: f32,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct ControlsConfig {}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct GeneralConfig {
    pub language: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct VisualConfig {
    pub mode: Mode,
    pub resolution: (u32, u32),
    pub vsync: bool,
}

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize)]
pub enum Mode {
    Borderless,
    Fullscreen,
    #[default]
    Windowed,
}

impl Default for VisualConfig {
    fn default() -> Self {
        Self {
            mode: Mode::Windowed,
            resolution: (1080, 720),
            vsync: true,
        }
    }
}
