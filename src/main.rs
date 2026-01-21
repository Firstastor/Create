use bevy::{prelude::*, window::*};

use crate::cameras::CameraRTSPlugin;
use crate::main_game::MainGamePlugin;
use crate::main_menu::MainMenuPlugin;
use crate::pause_menu::PauseMenuPlugin;
use crate::states::*;

mod cameras;
mod constants;
mod main_game;
mod main_menu;
mod pause_menu;
mod states;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoVsync,
                position: WindowPosition::Centered(MonitorSelection::Primary),
                resolution: WindowResolution::new(1600, 900),
                title: "Create".to_string(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .init_state::<GameState>()
        .add_plugins(CameraRTSPlugin)
        .add_plugins(MainGamePlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(PauseMenuPlugin)
        .run();
}
