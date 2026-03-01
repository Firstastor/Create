use bevy::{asset::AssetMetaCheck, prelude::*, window::*};
use std::fs;

pub mod game_config;
pub use game_config::*;

pub struct ConfigsPlugin;

impl Plugin for ConfigsPlugin {
    fn build(&self, app: &mut App) {
        let game_config_bytes = fs::read("config.toml").expect("Failed to read config file");
        let game_config: GameConfig =
            toml::from_slice(&game_config_bytes).expect("Failed to parse config file");

        let mode = match game_config.visual.mode {
            Mode::Borderless => WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            Mode::Fullscreen => {
                WindowMode::Fullscreen(MonitorSelection::Primary, VideoModeSelection::Current)
            }
            Mode::Windowed => WindowMode::Windowed,
        };
        let resolution = game_config.visual.resolution;
        let vsync = match game_config.visual.vsync {
            true => PresentMode::AutoVsync,
            false => PresentMode::AutoNoVsync,
        };

        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: mode,
                        present_mode: vsync,
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: WindowResolution::new(resolution.0, resolution.1),
                        title: "Create".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(game_config);
    }
}
