use bevy::app::AppExit;
use bevy::prelude::*;

use crate::ui::AppState;

#[derive(Component)]
pub struct PauseMenuCamera;

#[derive(Component)]
pub struct PauseMenuRoot;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum PauseMenuAction {
    Task,
    System,
    Save,
    Load,
    Settings,
    Exit,
    Quit,
}

pub fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn((Camera2d, PauseMenuCamera));
    commands
        .spawn((
            Name::new("Pause Menu Root"),
            PauseMenuRoot,
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            BackgroundColor(Color::srgba(0.03, 0.03, 0.03, 0.9)),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: percent(100),
                        height: percent(6),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::FlexStart,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.06, 0.06, 0.06)),
                ))
                .with_children(|top| {
                    spawn_tab_button(top, "Task", PauseMenuAction::Task);
                    spawn_tab_button(top, "System", PauseMenuAction::System);
                });

            parent
                .spawn((
                    Node {
                        width: percent(100),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Stretch,
                        row_gap: Val::Px(12.0),
                        padding: UiRect::all(Val::Px(16.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.04, 0.04, 0.04)),
                    Visibility::Visible,
                ))
                .with_children(|panel| {
                    panel
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(46.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.08, 0.08, 0.08)),
                            Button,
                            PauseMenuAction::Settings,
                        ))
                        .with_children(|button| {
                            spawn_button_label(button, "Settings");
                        });

                    panel
                        .spawn((Node {
                            width: Val::Percent(100.0),
                            height: Val::Auto,
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            column_gap: Val::Px(12.0),
                            ..default()
                        },))
                        .with_children(|row| {
                            spawn_action_button(row, "Save", PauseMenuAction::Save);
                            spawn_action_button(row, "Load", PauseMenuAction::Load);
                        });

                    panel
                        .spawn((Node {
                            width: Val::Percent(100.0),
                            height: Val::Auto,
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            column_gap: Val::Px(12.0),
                            ..default()
                        },))
                        .with_children(|row| {
                            spawn_action_button(row, "Exit", PauseMenuAction::Exit);
                            spawn_action_button(row, "Quit", PauseMenuAction::Quit);
                        });
                });
        });
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    root: Query<Entity, Or<(With<PauseMenuCamera>, With<PauseMenuRoot>)>>,
) {
    for entity in root.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn pause_menu_input(
    mut interactions: Query<(&Interaction, &PauseMenuAction), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut writer: MessageWriter<AppExit>,
) {
    for (interaction, action) in &mut interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match action {
            PauseMenuAction::Task => {
                // clickable but no action
            }
            PauseMenuAction::System => {
                // clickable but no action
            }
            PauseMenuAction::Save => {
                // clickable but no action
            }
            PauseMenuAction::Load => {
                // clickable but no action
            }
            PauseMenuAction::Settings => {
                next_state.set(AppState::Settings);
            }
            PauseMenuAction::Exit => {
                next_state.set(AppState::MainMenu);
            }
            PauseMenuAction::Quit => {
                writer.write(AppExit::Success);
            }
        }
    }
}

fn spawn_tab_button(parent: &mut ChildSpawnerCommands, label: &str, action: PauseMenuAction) {
    parent
        .spawn((
            Button,
            action,
            Node {
                width: percent(50),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(px(2)),
                border_radius: BorderRadius::all(px(8)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.08, 0.08)),
        ))
        .with_children(|button| {
            spawn_button_label(button, label);
        });
}

fn spawn_action_button(parent: &mut ChildSpawnerCommands, label: &str, action: PauseMenuAction) {
    parent
        .spawn((
            Button,
            action,
            Node {
                width: Val::Percent(50.0),
                height: Val::Px(46.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.08, 0.08)),
        ))
        .with_children(|button| {
            spawn_button_label(button, label);
        });
}

fn spawn_button_label(parent: &mut ChildSpawnerCommands, label: &str) {
    parent.spawn((
        Text::new(label),
        TextColor(Color::srgb(0.88, 0.88, 0.88)),
        TextFont {
            font_size: 20.0,
            ..default()
        },
    ));
}
