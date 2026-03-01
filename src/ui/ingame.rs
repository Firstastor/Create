use bevy::prelude::*;

use crate::ui::AppState;

#[derive(Component, Default)]
pub struct InGameCamera;

#[derive(Component)]
#[require(Camera3d)]
pub struct InGame3dCamera;

#[derive(Component)]
#[require(Camera2d)]
pub struct InGameUiCamera;

#[derive(Component)]
pub struct InGameRoot;

pub fn toggle_pause_menu(
    input: Res<ButtonInput<KeyCode>>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if !input.just_pressed(KeyCode::Escape) {
        return;
    }

    match state.get() {
        AppState::InGame => next_state.set(AppState::PauseMenu),
        AppState::PauseMenu => next_state.set(AppState::InGame),
        _ => {}
    }
}

pub fn spawn_ingame(mut commands: Commands, existing_root: Query<Entity, With<InGameRoot>>) {
    if !existing_root.is_empty() {
        return;
    }
    commands.spawn((
        InGameCamera,
        InGame3dCamera,
        Camera {
            order: 0,
            ..default()
        },
    ));
    commands.spawn((
        InGameCamera,
        InGameUiCamera,
        Camera {
            order: 1,
            ..default()
        },
    ));

    commands.spawn((
        Name::new("In-Game Root"),
        InGameRoot,
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
    ));
}

pub fn despawn_ingame(
    mut commands: Commands,
    root: Query<Entity, Or<(With<InGameCamera>, With<InGameRoot>)>>,
) {
    println!("输出");
    for entity in root.iter() {
        commands.entity(entity).despawn();
    }
}
