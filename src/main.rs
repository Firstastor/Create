use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::default(),
                mode: bevy::window::WindowMode::default(),
                position: bevy::window::WindowPosition::Centered(MonitorSelection::Primary),
                resolution: bevy::window::WindowResolution::default(),
                title: "Create".to_string(),
                ..default()
            }),
            ..default()
        }))
        .run();
}
