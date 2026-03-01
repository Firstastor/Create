use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::ecs::message::MessageReader;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::window::{
    MonitorSelection, PresentMode, PrimaryWindow, VideoModeSelection, WindowMode, WindowResolution,
};
use std::fs;
use std::time::SystemTime;

use crate::configs::{GameConfig, Mode};
use crate::ui::AppState;

#[derive(Component)]
pub struct SettingsRoot;

#[derive(Component)]
pub struct SettingsCamera;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum SettingsAction {
    ReturnMainMenu,
    ReturnPauseMenu,
    ReloadConfig,
    MasterVolumeDec,
    MasterVolumeInc,
    MusicVolumeDec,
    MusicVolumeInc,
    LanguageEnglish,
    LanguageChinese,
    ModeWindowed,
    ModeFullscreen,
    ModeBorderless,
    VsyncOn,
    VsyncOff,
    ResolutionFocusX,
    ResolutionFocusY,
    ResolutionApply,
}

#[derive(Component)]
pub struct ProgressBarFill {
    pub kind: AudioKind,
}

#[derive(Component)]
pub struct ValueLabel {
    pub kind: ValueKind,
}

#[derive(Component)]
pub struct ResolutionInputXLabel;

#[derive(Component)]
pub struct ResolutionInputYLabel;

#[derive(Resource, Default)]
pub struct ResolutionInputState {
    pub x: String,
    pub y: String,
    pub focused: Option<ResolutionField>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ResolutionField {
    X,
    Y,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AudioKind {
    Master,
    Music,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ValueKind {
    MasterVolume,
    MusicVolume,
    Language,
    Mode,
    Vsync,
    Resolution,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RadioGroup {
    Language,
    Mode,
    Vsync,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RadioValue {
    LanguageEnglish,
    LanguageChinese,
    ModeWindowed,
    ModeFullscreen,
    ModeBorderless,
    VsyncOn,
    VsyncOff,
}

pub fn spawn_settings(
    mut commands: Commands,
    config: Res<GameConfig>,
    existing_cameras: Query<Entity, With<Camera2d>>,
) {
    if existing_cameras.is_empty() {
        commands.spawn((Camera2d, SettingsCamera));
    }

    commands.insert_resource(ResolutionInputState {
        x: config.visual.resolution.0.to_string(),
        y: config.visual.resolution.1.to_string(),
        focused: None,
    });

    let root = commands
        .spawn((
            SettingsRoot,
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Stretch,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((Node {
                    width: percent(30),
                    height: percent(100),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Stretch,
                    justify_content: JustifyContent::FlexStart,
                    padding: UiRect::all(Val::Px(24.0)),
                    row_gap: Val::Px(16.0),
                    ..default()
                },))
                .with_children(|left| {
                    left.spawn((
                        Text::new("Settings"),
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                    ));

                    spawn_category_button(left, "Audio");
                    spawn_category_button(left, "Controls");
                    spawn_category_button(left, "General");
                    spawn_category_button(left, "Visual");
                });

            parent
                .spawn((
                    Node {
                        width: percent(70),
                        height: percent(100),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Stretch,
                        justify_content: JustifyContent::FlexStart,
                        padding: UiRect::all(px(24)),
                        row_gap: Val::Px(18.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.03, 0.03, 0.03)),
                ))
                .with_children(|right| {
                    // audio
                    right.spawn((
                        Text::new("audio"),
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        TextFont {
                            font_size: 22.0,
                            ..default()
                        },
                    ));

                    spawn_audio_row(
                        right,
                        "master_volume",
                        AudioKind::Master,
                        config.audio.master_volume,
                    );
                    spawn_audio_row(
                        right,
                        "music_volume",
                        AudioKind::Music,
                        config.audio.music_volume,
                    );

                    // controls (empty struct)
                    right.spawn((
                        Text::new("controls"),
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        TextFont {
                            font_size: 22.0,
                            ..default()
                        },
                    ));

                    right.spawn((
                        Text::new("(none)"),
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                    ));

                    // general
                    right.spawn((
                        Text::new("general"),
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        TextFont {
                            font_size: 22.0,
                            ..default()
                        },
                    ));

                    spawn_language_row(right, &config.general.language);

                    // visual
                    right.spawn((
                        Text::new("visual"),
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        TextFont {
                            font_size: 22.0,
                            ..default()
                        },
                    ));

                    spawn_mode_row(right, config.visual.mode);
                    spawn_resolution_row(right, config.visual.resolution);
                    spawn_vsync_row(right, config.visual.vsync);

                    right
                        .spawn((Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(48.0),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            column_gap: Val::Px(12.0),
                            ..default()
                        },))
                        .with_children(|row| {
                            spawn_action_button(row, "Hot Reload", SettingsAction::ReloadConfig);
                            spawn_action_button(
                                row,
                                "Back to Main Menu",
                                SettingsAction::ReturnMainMenu,
                            );
                            spawn_action_button(
                                row,
                                "Back to Pause Menu",
                                SettingsAction::ReturnPauseMenu,
                            );
                        });
                });
        });
}

pub fn despawn_settings(
    mut commands: Commands,
    root: Query<Entity, Or<(With<SettingsRoot>, With<SettingsCamera>)>>,
) {
    for entity in root.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn settings_input(
    mut interactions: Query<(&Interaction, &SettingsAction), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut config: ResMut<GameConfig>,
    mut labels: Query<&mut Text, With<ValueLabel>>,
    mut bars: Query<(&mut Node, &ProgressBarFill)>,
    mut resolution_state: ResMut<ResolutionInputState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut changed = false;

    for (interaction, action) in &mut interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match action {
            SettingsAction::ReturnMainMenu => {
                next_state.set(AppState::MainMenu);
            }
            SettingsAction::ReturnPauseMenu => {
                next_state.set(AppState::PauseMenu);
            }
            SettingsAction::ReloadConfig => {
                if let Ok(bytes) = fs::read("config.toml") {
                    if let Ok(new_config) = toml::from_slice::<GameConfig>(&bytes) {
                        *config = new_config;
                        resolution_state.x = config.visual.resolution.0.to_string();
                        resolution_state.y = config.visual.resolution.1.to_string();
                        changed = true;
                    }
                }
            }
            SettingsAction::MasterVolumeDec => {
                config.audio.master_volume = (config.audio.master_volume - 0.05).clamp(0.0, 1.0);
                changed = true;
            }
            SettingsAction::MasterVolumeInc => {
                config.audio.master_volume = (config.audio.master_volume + 0.05).clamp(0.0, 1.0);
                changed = true;
            }
            SettingsAction::MusicVolumeDec => {
                config.audio.music_volume = (config.audio.music_volume - 0.05).clamp(0.0, 1.0);
                changed = true;
            }
            SettingsAction::MusicVolumeInc => {
                config.audio.music_volume = (config.audio.music_volume + 0.05).clamp(0.0, 1.0);
                changed = true;
            }
            SettingsAction::LanguageEnglish => {
                config.general.language = "en".to_string();
                changed = true;
            }
            SettingsAction::LanguageChinese => {
                config.general.language = "zh".to_string();
                changed = true;
            }
            SettingsAction::ModeWindowed => {
                config.visual.mode = Mode::Windowed;
                changed = true;
            }
            SettingsAction::ModeFullscreen => {
                config.visual.mode = Mode::Fullscreen;
                changed = true;
            }
            SettingsAction::ModeBorderless => {
                config.visual.mode = Mode::Borderless;
                changed = true;
            }
            SettingsAction::VsyncOn => {
                config.visual.vsync = true;
                changed = true;
            }
            SettingsAction::VsyncOff => {
                config.visual.vsync = false;
                changed = true;
            }
            SettingsAction::ResolutionFocusX => {
                resolution_state.focused = Some(ResolutionField::X);
            }
            SettingsAction::ResolutionFocusY => {
                resolution_state.focused = Some(ResolutionField::Y);
            }
            SettingsAction::ResolutionApply => {
                if apply_resolution_input(&mut config, &resolution_state, &windows) {
                    changed = true;
                }
            }
        }
    }

    if changed {
        apply_config_to_labels(&config, &mut labels);
        apply_audio_bars(&config, &mut bars);
        apply_config_runtime(&config, &mut windows);
    }
}

pub fn settings_text_input(
    mut keyboard_events: MessageReader<KeyboardInput>,
    input: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<GameConfig>,
    mut resolution_state: ResMut<ResolutionInputState>,
    mut labels: Query<&mut Text, With<ValueLabel>>,
    mut bars: Query<(&mut Node, &ProgressBarFill)>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Some(focused) = resolution_state.focused else {
        return;
    };

    let target = match focused {
        ResolutionField::X => &mut resolution_state.x,
        ResolutionField::Y => &mut resolution_state.y,
    };

    for ev in keyboard_events.read() {
        let Some(text) = ev.text.as_deref() else {
            continue;
        };

        for ch in text.chars() {
            if ch.is_ascii_digit() {
                target.push(ch);
            }
        }
    }

    if input.just_pressed(KeyCode::Backspace) {
        target.pop();
    }

    if input.just_pressed(KeyCode::Enter) {
        if apply_resolution_input(&mut config, &resolution_state, &windows) {
            apply_config_to_labels(&config, &mut labels);
            apply_audio_bars(&config, &mut bars);
            apply_config_runtime(&config, &mut windows);
        }
        resolution_state.focused = None;
    }
}

pub fn hot_reload_config(
    mut config: ResMut<GameConfig>,
    mut last_modified: Local<Option<SystemTime>>,
    mut labels: Query<&mut Text, With<ValueLabel>>,
    mut bars: Query<(&mut Node, &ProgressBarFill)>,
    mut resolution_state: ResMut<ResolutionInputState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(metadata) = fs::metadata("config.toml") else {
        return;
    };

    let Ok(modified) = metadata.modified() else {
        return;
    };

    if last_modified.map(|time| time >= modified).unwrap_or(false) {
        return;
    }

    let Ok(bytes) = fs::read("config.toml") else {
        return;
    };

    let Ok(new_config) = toml::from_slice::<GameConfig>(&bytes) else {
        return;
    };

    *config = new_config;
    resolution_state.x = config.visual.resolution.0.to_string();
    resolution_state.y = config.visual.resolution.1.to_string();
    *last_modified = Some(modified);
    apply_config_to_labels(&config, &mut labels);
    apply_audio_bars(&config, &mut bars);
    apply_config_runtime(&config, &mut windows);
}

pub fn save_config_on_exit(config: Res<GameConfig>) {
    if let Ok(serialized) = toml::to_string_pretty(&*config) {
        let _ = fs::write("config.toml", serialized);
    }
}

fn spawn_category_button(parent: &mut ChildSpawnerCommands, label: &str) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
            BackgroundColor(Color::srgb(0.07, 0.07, 0.07)),
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
            ));
        });
}

fn spawn_audio_row(parent: &mut ChildSpawnerCommands, label: &str, kind: AudioKind, value: f32) {
    parent
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Px(52.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            column_gap: Val::Px(12.0),
            ..default()
        },))
        .with_children(|row| {
            row.spawn((
                Text::new(label),
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
            ));

            row.spawn((
                Node {
                    width: Val::Percent(55.0),
                    height: Val::Px(10.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
                BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            ))
            .with_children(|bar| {
                bar.spawn((
                    ProgressBarFill { kind },
                    Node {
                        width: Val::Percent((value * 100.0).clamp(0.0, 100.0)),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.6, 0.6, 0.6)),
                ));
            });

            row.spawn((
                Text::new(format!("{:.0}%", value * 100.0)),
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                ValueLabel {
                    kind: match kind {
                        AudioKind::Master => ValueKind::MasterVolume,
                        AudioKind::Music => ValueKind::MusicVolume,
                    },
                },
            ));

            row.spawn((
                Button,
                match kind {
                    AudioKind::Master => SettingsAction::MasterVolumeDec,
                    AudioKind::Music => SettingsAction::MusicVolumeDec,
                },
                Text::new("-"),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
            ));
            row.spawn((
                Button,
                match kind {
                    AudioKind::Master => SettingsAction::MasterVolumeInc,
                    AudioKind::Music => SettingsAction::MusicVolumeInc,
                },
                Text::new("+"),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
            ));
        });
}

fn spawn_language_row(parent: &mut ChildSpawnerCommands, language: &str) {
    parent
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Px(48.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            column_gap: Val::Px(12.0),
            ..default()
        },))
        .with_children(|row| {
            row.spawn((
                Text::new("language"),
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
            ));

            spawn_radio(
                row,
                "English",
                RadioGroup::Language,
                RadioValue::LanguageEnglish,
            );
            spawn_radio(
                row,
                "Chinese",
                RadioGroup::Language,
                RadioValue::LanguageChinese,
            );

            row.spawn((
                Text::new(format_language(language)),
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                ValueLabel {
                    kind: ValueKind::Language,
                },
            ));
        });
}

fn spawn_mode_row(parent: &mut ChildSpawnerCommands, mode: Mode) {
    parent
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Px(48.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            column_gap: Val::Px(12.0),
            ..default()
        },))
        .with_children(|row| {
            row.spawn((
                Text::new("mode"),
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
            ));

            spawn_radio(row, "Windowed", RadioGroup::Mode, RadioValue::ModeWindowed);
            spawn_radio(
                row,
                "Fullscreen",
                RadioGroup::Mode,
                RadioValue::ModeFullscreen,
            );
            spawn_radio(
                row,
                "Borderless",
                RadioGroup::Mode,
                RadioValue::ModeBorderless,
            );

            row.spawn((
                Text::new(format_window_mode(mode)),
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                ValueLabel {
                    kind: ValueKind::Mode,
                },
            ));
        });
}

fn spawn_vsync_row(parent: &mut ChildSpawnerCommands, vsync: bool) {
    parent
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Px(48.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            column_gap: Val::Px(12.0),
            ..default()
        },))
        .with_children(|row| {
            row.spawn((
                Text::new("vsync"),
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
            ));

            spawn_radio(row, "On", RadioGroup::Vsync, RadioValue::VsyncOn);
            spawn_radio(row, "Off", RadioGroup::Vsync, RadioValue::VsyncOff);

            row.spawn((
                Text::new(format_vsync(vsync)),
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                ValueLabel {
                    kind: ValueKind::Vsync,
                },
            ));
        });
}

fn spawn_resolution_row(parent: &mut ChildSpawnerCommands, resolution: (u32, u32)) {
    parent
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Px(48.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            column_gap: Val::Px(12.0),
            ..default()
        },))
        .with_children(|row| {
            row.spawn((
                Text::new("resolution"),
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
            ));

            row.spawn((
                Button,
                SettingsAction::ResolutionFocusX,
                Text::new(resolution.0.to_string()),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                ResolutionInputXLabel,
            ));
            row.spawn((
                Text::new("x"),
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
            ));
            row.spawn((
                Button,
                SettingsAction::ResolutionFocusY,
                Text::new(resolution.1.to_string()),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                ResolutionInputYLabel,
            ));
            row.spawn((
                Button,
                SettingsAction::ResolutionApply,
                Text::new("Apply"),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
            ));
            row.spawn((
                Text::new(format_resolution(resolution)),
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                ValueLabel {
                    kind: ValueKind::Resolution,
                },
            ));
        });
}

fn spawn_radio(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    group: RadioGroup,
    value: RadioValue,
) {
    let _ = group;
    parent.spawn((
        Button,
        radio_action(value),
        Node {
            height: Val::Px(30.0),
            padding: UiRect::horizontal(Val::Px(8.0)),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        BorderColor::all(Color::srgb(0.35, 0.35, 0.35)),
        BackgroundColor(Color::srgb(0.08, 0.08, 0.08)),
        Text::new(label),
        TextColor(Color::srgb(0.88, 0.88, 0.88)),
        TextFont {
            font_size: 14.0,
            ..default()
        },
    ));
}

fn spawn_action_button(parent: &mut ChildSpawnerCommands, label: &str, action: SettingsAction) {
    parent
        .spawn((
            Button,
            action,
            Node {
                width: Val::Auto,
                height: Val::Px(34.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(12.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.08, 0.08)),
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextColor(Color::srgb(0.88, 0.88, 0.88)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
            ));
        });
}

fn apply_config_to_labels(config: &GameConfig, labels: &mut Query<&mut Text, With<ValueLabel>>) {
    for mut text in labels.iter_mut() {
        if text.0.contains("master_volume") {
            text.0 = format!("{:.0}%", config.audio.master_volume * 100.0);
        } else if text.0.contains("music_volume") {
            text.0 = format!("{:.0}%", config.audio.music_volume * 100.0);
        } else {
            // fallback by value kind labels
            text.0 = text.0.clone();
        }
    }
}

fn apply_audio_bars(config: &GameConfig, bars: &mut Query<(&mut Node, &ProgressBarFill)>) {
    for (mut node, fill) in bars.iter_mut() {
        let value = match fill.kind {
            AudioKind::Master => config.audio.master_volume,
            AudioKind::Music => config.audio.music_volume,
        };
        node.width = Val::Percent((value * 100.0).clamp(0.0, 100.0));
    }
}

fn apply_config_runtime(
    config: &GameConfig,
    windows: &mut Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(mut window) = windows.single_mut() else {
        return;
    };

    window.present_mode = if config.visual.vsync {
        PresentMode::AutoVsync
    } else {
        PresentMode::AutoNoVsync
    };

    window.mode = match config.visual.mode {
        Mode::Borderless => WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
        Mode::Fullscreen => {
            WindowMode::Fullscreen(MonitorSelection::Primary, VideoModeSelection::Current)
        }
        Mode::Windowed => WindowMode::Windowed,
    };

    if matches!(config.visual.mode, Mode::Windowed) {
        window.resolution =
            WindowResolution::new(config.visual.resolution.0, config.visual.resolution.1);
    }
}

fn apply_resolution_input(
    config: &mut GameConfig,
    state: &ResolutionInputState,
    windows: &Query<&mut Window, With<PrimaryWindow>>,
) -> bool {
    let Ok(window) = windows.single() else {
        return false;
    };

    let max_x = window.resolution.width() as u32;
    let max_y = window.resolution.height() as u32;

    let x = state.x.parse::<u32>().unwrap_or(config.visual.resolution.0);
    let y = state.y.parse::<u32>().unwrap_or(config.visual.resolution.1);

    let clamped_x = x.clamp(1, max_x.max(1));
    let clamped_y = y.clamp(1, max_y.max(1));

    if config.visual.resolution != (clamped_x, clamped_y) {
        config.visual.resolution = (clamped_x, clamped_y);
        true
    } else {
        false
    }
}

fn radio_action(value: RadioValue) -> SettingsAction {
    match value {
        RadioValue::LanguageEnglish => SettingsAction::LanguageEnglish,
        RadioValue::LanguageChinese => SettingsAction::LanguageChinese,
        RadioValue::ModeWindowed => SettingsAction::ModeWindowed,
        RadioValue::ModeFullscreen => SettingsAction::ModeFullscreen,
        RadioValue::ModeBorderless => SettingsAction::ModeBorderless,
        RadioValue::VsyncOn => SettingsAction::VsyncOn,
        RadioValue::VsyncOff => SettingsAction::VsyncOff,
    }
}

fn format_vsync(enabled: bool) -> String {
    if enabled {
        "On".to_string()
    } else {
        "Off".to_string()
    }
}

fn format_window_mode(mode: Mode) -> String {
    match mode {
        Mode::Windowed => "Windowed",
        Mode::Fullscreen => "Fullscreen",
        Mode::Borderless => "Borderless",
    }
    .to_string()
}

fn format_resolution(resolution: (u32, u32)) -> String {
    format!("{} x {}", resolution.0, resolution.1)
}

fn format_language(language: &str) -> String {
    match language {
        "en" => "English".to_string(),
        "zh" => "Chinese".to_string(),
        _ => language.to_string(),
    }
}
