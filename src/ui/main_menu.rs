use bevy::app::AppExit;
use bevy::prelude::*;

use crate::ui::AppState;

#[derive(Component)]
pub struct MainMenuRoot;

#[derive(Component)]
pub struct MainMenuCamera;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum MainMenuButton {
    Start,
    Load,
    Settings,
    Exit,
}

pub fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((Camera2d, MainMenuCamera));
    commands
        .spawn((
            Name::new("Main Menu Root"),
            MainMenuRoot,
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: px(12),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Create"),
                TextColor(Color::srgb(0.88, 0.88, 0.88)),
                TextFont {
                    font_size: 28.0,
                    weight: FontWeight::BOLD,
                    ..default()
                },
            ));
            parent.spawn(Node {
                width: percent(100),
                height: px(24),
                ..default()
            });
            spawn_menu_button(parent, "Start", MainMenuButton::Start);
            spawn_menu_button(parent, "Load", MainMenuButton::Load);
            spawn_menu_button(parent, "Settings", MainMenuButton::Settings);
            spawn_menu_button(parent, "Exit", MainMenuButton::Exit);
        });
}

pub fn despawn_main_menu(
    mut commands: Commands,
    root: Query<Entity, Or<(With<MainMenuRoot>, With<MainMenuCamera>)>>,
) {
    for entity in root.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn_menu_button(parent: &mut ChildSpawnerCommands, label: &str, button_type: MainMenuButton) {
    parent
        .spawn((
            Button,
            button_type,
            Node {
                width: px(240),
                height: px(48),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(px(2)),
                border_radius: BorderRadius::all(px(8)),
                ..default()
            },
            BorderColor::all(Color::srgb(0.08, 0.08, 0.08)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextColor(Color::srgb(0.88, 0.88, 0.88)),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
            ));
        });
}

pub fn main_menu_input(
    mut interactions: Query<(&Interaction, &MainMenuButton), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut writer: MessageWriter<AppExit>,
) {
    for (interaction, button) in &mut interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match button {
            MainMenuButton::Start => {
                next_state.set(AppState::InGame);
            }
            MainMenuButton::Load => {
                // clickable but no action
            }
            MainMenuButton::Settings => {
                next_state.set(AppState::Settings);
            }
            MainMenuButton::Exit => {
                writer.write(AppExit::Success);
            }
        }
    }
}
