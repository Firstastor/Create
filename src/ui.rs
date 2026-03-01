use bevy::prelude::*;

mod ingame;
use ingame::*;
mod main_menu;
use main_menu::*;
mod pause_menu;
use pause_menu::*;
mod settings;
use settings::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            // In-Game
            .add_systems(OnEnter(AppState::InGame), spawn_ingame)
            .add_systems(OnExit(AppState::InGame), despawn_ingame)
            .add_systems(
                Update,
                toggle_pause_menu.run_if(in_state(AppState::InGame)),
            )
            // Main Menu
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
            .add_systems(
                Update,
                main_menu_input.run_if(in_state(AppState::MainMenu)),
            )
            // Pause Menu
            .add_systems(OnEnter(AppState::PauseMenu), spawn_pause_menu)
            .add_systems(OnExit(AppState::PauseMenu), despawn_pause_menu)
            .add_systems(
                Update,
                pause_menu_input.run_if(in_state(AppState::PauseMenu)),
            )
            // Settings
            .add_systems(OnEnter(AppState::Settings), spawn_settings)
            .add_systems(OnExit(AppState::Settings), despawn_settings)
            .add_systems(
                Update,
                (settings_input, hot_reload_config)
                    .run_if(in_state(AppState::Settings)),
            ).add_systems(Update, log_state_changes)
            .add_systems(OnExit(AppState::Settings), save_config_on_exit);
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    InGame,
    #[default]
    MainMenu,
    PauseMenu,
    Settings,
}

fn log_state_changes(mut transitions: MessageReader<StateTransitionEvent<AppState>>) {
    for transition in transitions.read() {
        println!(
            "状态切换: {:?} => {:?}", 
            transition.exited, 
            transition.entered
        );
    }
}