use bevy::prelude::*;

use crate::{constants::*, states::AppState};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), clean_main_menu);
    }
}

#[derive(Component)]
struct MainMenuRoot;

fn spawn_main_menu(mut commands: Commands) {
    commands
        .spawn((
            MainMenuRoot,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: percent(100),
                height: percent(100),
                row_gap: px(16),
                ..default()
            },
            BackgroundColor(COLOR_BG),
        ))
        .with_children(|parent| {
            // Title
            parent
                .spawn((Node {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: px(300),
                    height: px(120),
                    margin: UiRect::bottom(px(30)),
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Create"),
                        TextColor(COLOR_PRIMARY),
                        TextFont::default().with_font_size(40.),
                    ));
                });
            // New
            main_menu_button(parent, 200., 50., "New", |commands| {
                commands.set_state(AppState::Gaming);
            });
            // Quit
            main_menu_button(parent, 200., 50., "Quit", |commands| {
                commands.write_message(AppExit::Success);
            });
        });
}

fn clean_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn main_menu_button<F>(
    parent: &mut ChildSpawnerCommands,
    width: f32,
    height: f32,
    text: &str,
    on_click: F,
) where
    F: Fn(&mut Commands) + Send + Sync + 'static,
{
    parent
        .spawn((
            Button,
            Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: px(width),
                height: px(height),
                border: UiRect::all(px(2)),
                border_radius: BorderRadius::all(px(4)),
                ..default()
            },
            BorderColor::all(COLOR_BORDER),
        ))
        .with_children(|parent| {
            parent.spawn((Text::new(text), TextColor(COLOR_TEXT)));
        })
        .observe(on_button_hover)
        .observe(on_button_out)
        .observe(on_button_press)
        .observe(move |_: On<Pointer<Click>>, mut commands: Commands| {
            on_click(&mut commands);
        });
}

fn on_button_hover(trigger: On<Pointer<Over>>, mut query: Query<&mut BorderColor>) {
    if let Ok(mut border) = query.get_mut(trigger.event_target()) {
        *border = BorderColor::all(COLOR_PRIMARY_HOVER);
    }
}

fn on_button_out(trigger: On<Pointer<Out>>, mut query: Query<&mut BorderColor>) {
    if let Ok(mut border) = query.get_mut(trigger.event_target()) {
        *border = BorderColor::all(COLOR_BORDER);
    }
}

fn on_button_press(trigger: On<Pointer<Press>>, mut query: Query<&mut BorderColor>) {
    if let Ok(mut border) = query.get_mut(trigger.event_target()) {
        *border = BorderColor::all(COLOR_PRIMARY_ACTIVE);
    }
}