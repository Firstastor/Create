use crate::constants::*;
use crate::states::*;
use bevy::prelude::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_esc_input)
            .add_systems(OnEnter(GameState::Paused), spawn_pause_menu)
            .add_systems(OnExit(GameState::Paused), clean_pause_menu);
    }
}

pub fn handle_esc_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    game_state: Res<State<GameState>>,
) {
    if !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }

    match app_state.get() {
        AppState::Gaming => match game_state.get() {
            GameState::Normal => {
                commands.set_state(GameState::Paused);
            }
            GameState::Paused => {
                commands.set_state(GameState::Normal);
            }
        },
        AppState::MainMenu => {}
    }
}

#[derive(Component)]
struct PauseMenuRoot;

#[derive(Component, Clone, Copy)]
enum PauseMenuAction {
    Resume,
    MainMenu,
    ExitGame,
}

fn spawn_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            PauseMenuRoot,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: percent(100),
                height: percent(100),
                row_gap: px(12),
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            GlobalZIndex(100),
        ))
        .with_children(|parent| {
            // Panel
            parent
                .spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(px(30)),
                        row_gap: px(12),
                        border: UiRect::all(px(2)),
                        border_radius: BorderRadius::all(px(12)),
                        ..default()
                    },
                    BackgroundColor(COLOR_SIDE),
                    BorderColor::all(COLOR_BORDER),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new("Paused"),
                        TextColor(COLOR_TEXT_LIGHT),
                        TextFont::default().with_font_size(28.0),
                        Node {
                            margin: UiRect::bottom(px(16)),
                            ..default()
                        },
                    ));

                    // Buttons
                    spawn_pause_menu_button(parent, 200.0, 40.0, "Resume", PauseMenuAction::Resume);
                    spawn_pause_menu_button(
                        parent,
                        200.0,
                        40.0,
                        "Main Menu",
                        PauseMenuAction::MainMenu,
                    );
                    spawn_pause_menu_button(parent, 200.0, 40.0, "Quit", PauseMenuAction::ExitGame);
                });
        });
}

fn clean_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn spawn_pause_menu_button(
    parent: &mut ChildSpawnerCommands,
    width: f32,
    height: f32,
    text: &str,
    action: PauseMenuAction,
) {
    parent
        .spawn((
            Button,
            action,
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
        .observe(on_button_click);
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

fn on_button_click(
    trigger: On<Pointer<Click>>,
    query: Query<&PauseMenuAction>,
    mut commands: Commands,
) {
    let Ok(action) = query.get(trigger.event_target()) else {
        return;
    };

    match action {
        PauseMenuAction::Resume => {
            commands.set_state(GameState::Normal);
        }
        PauseMenuAction::MainMenu => {
            commands.set_state(AppState::MainMenu);
        }
        PauseMenuAction::ExitGame => {
            commands.write_message(AppExit::Success);
        }
    }
}
