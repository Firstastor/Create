use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};

use crate::states::*;

pub struct CameraRTSPlugin;

impl Plugin for CameraRTSPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, confine_cursor))
            .add_systems(OnEnter(AppState::Gaming), confine_cursor)
            .add_systems(OnExit(AppState::Gaming), release_cursor)
            .add_systems(OnEnter(GameState::Paused), release_cursor)
            .add_systems(OnExit(GameState::Paused), confine_cursor)
            .add_systems(Update, (camera_rts_control_system, camera_rts_smooth_system));
    }
}

#[derive(Component)]
pub struct CameraRTS {
    pub focus: Vec3,
    pub target_focus: Vec3,
    pub radius: f32,
    pub target_radius: f32,
    pub yaw: f32,
    pub target_yaw: f32,
    pub pitch: f32,
    pub pitch_min: f32,
    pub pitch_max: f32,
    pub pan_speed: f32,
    pub edge_pan_speed: f32,
    pub edge_pan_threshold: f32,
    pub zoom_speed: f32,
    pub rotation_speed: f32,
    pub smoothness: f32,
    pub zoom_min: f32,
    pub zoom_max: f32,
}

impl CameraRTS {
    /// Create CameraRTS with initial values calculated from camera transform
    pub fn from_transform(translation: Vec3, focus: Vec3) -> Self {
        let delta = translation - focus;
        let radius = delta.length().max(0.05);
        let yaw = delta.x.atan2(delta.z);
        let pitch = (delta.y / radius).asin();

        Self {
            focus,
            target_focus: focus,
            radius,
            target_radius: radius,
            yaw,
            target_yaw: yaw,
            pitch,
            pitch_min: 0.05,
            pitch_max: 0.85,
            pan_speed: 70.0,
            edge_pan_speed: 50.0,
            edge_pan_threshold: 30.0,
            zoom_speed: 4.0,
            rotation_speed: 0.005,
            smoothness: 0.1,
            zoom_min: 5.0,
            zoom_max: 100.0,
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Camera {
            order: 1,
            ..default()
        },
    ));

    let initial_pos = Vec3::new(0.0, 15.0, 15.0);
    let focus = Vec3::ZERO;
    commands.spawn((
        Camera3d::default(),
        CameraRTS::from_transform(initial_pos, focus),
        Transform::from_xyz(initial_pos.x, initial_pos.y, initial_pos.z).looking_at(focus, Vec3::Y),
    ));
}

/// Confine cursor to the window
fn confine_cursor(mut cursor_query: Query<&mut CursorOptions>) {
    if let Ok(mut cursor) = cursor_query.single_mut() {
        cursor.grab_mode = CursorGrabMode::Confined;
    }
}

/// Release cursor from the window
fn release_cursor(mut cursor_query: Query<&mut CursorOptions>) {
    if let Ok(mut cursor) = cursor_query.single_mut() {
        cursor.grab_mode = CursorGrabMode::None;
    }
}

pub fn camera_rts_control_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut scroll_events: MessageReader<MouseWheel>,
    windows: Query<&Window>,
    mut query: Query<(&mut CameraRTS, &GlobalTransform, &Camera), With<CameraRTS>>,
) {
    let dt = time.delta_secs();

    for (mut cam, global_transform, camera) in query.iter_mut() {
        let mut pan_delta = Vec3::ZERO;

        if keyboard.pressed(KeyCode::KeyW) {
            pan_delta.z -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            pan_delta.z += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            pan_delta.x += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            pan_delta.x -= 1.0;
        }

        if let Ok(window) = windows.single() {
            if let Some(cursor_pos) = window.cursor_position() {
                let w = window.width();
                let h = window.height();
                let threshold = cam.edge_pan_threshold;

                if cursor_pos.x < threshold {
                    pan_delta.x += 1.0;
                } else if cursor_pos.x > w - threshold {
                    pan_delta.x -= 1.0;
                }

                if cursor_pos.y < threshold {
                    pan_delta.z -= 1.0;
                } else if cursor_pos.y > h - threshold {
                    pan_delta.z += 1.0;
                }
            }
        }

        if pan_delta.length_squared() > 0.0 {
            let forward = Vec3::new(-cam.yaw.sin(), 0.0, -cam.yaw.cos());
            let right = Vec3::new(forward.z, 0.0, -forward.x);
            let speed = if keyboard.any_pressed([
                KeyCode::KeyW,
                KeyCode::KeyA,
                KeyCode::KeyS,
                KeyCode::KeyD,
            ]) {
                cam.pan_speed
            } else {
                cam.edge_pan_speed
            };
            let movement = (forward * -pan_delta.z + right * pan_delta.x) * speed * dt;
            cam.target_focus += movement;
        }

        for event in scroll_events.read() {
            let scroll_amount = event.y;
            if scroll_amount.abs() > 0.0 {
                let zoom_delta = scroll_amount * cam.zoom_speed;
                let new_radius = (cam.target_radius - zoom_delta).clamp(cam.zoom_min, cam.zoom_max);
                let actual_zoom = cam.target_radius - new_radius;

                if actual_zoom.abs() > 0.001 {
                    if let Ok(window) = windows.single() {
                        if let Some(cursor_pos) = window.cursor_position() {
                            if let Ok(ray) = camera.viewport_to_world(global_transform, cursor_pos)
                            {
                                if ray.direction.y.abs() >= 0.0001 {
                                    let t = (cam.focus.y - ray.origin.y) / ray.direction.y;
                                    if t >= 0.0 {
                                        let cursor_world = ray.origin + ray.direction * t;
                                        let zoom_ratio = actual_zoom / cam.target_radius;
                                        let focus_to_cursor = cursor_world - cam.target_focus;
                                        cam.target_focus += focus_to_cursor * zoom_ratio;
                                    }
                                }
                            }
                        }
                    }
                }
                cam.target_radius = new_radius;
            }
        }

        if mouse_button.pressed(MouseButton::Middle) {
            for event in mouse_motion.read() {
                cam.target_yaw -= event.delta.x * cam.rotation_speed;
            }
        } else {
            mouse_motion.clear();
        }
    }
}

/// Handles camera smooth interpolation
pub fn camera_rts_smooth_system(
    time: Res<Time>,
    mut query: Query<(&mut CameraRTS, &mut Transform), With<CameraRTS>>,
) {
    let dt = time.delta_secs();

    for (mut cam, mut transform) in query.iter_mut() {
        let lerp_factor = 1.0 - cam.smoothness.powi(7).powf(dt);
        cam.yaw = lerp_snap(cam.yaw, cam.target_yaw, lerp_factor);
        cam.radius = lerp_snap(cam.radius, cam.target_radius, lerp_factor);
        cam.focus = lerp_snap_vec3(cam.focus, cam.target_focus, lerp_factor);

        let zoom_ratio = (cam.radius - cam.zoom_min) / (cam.zoom_max - cam.zoom_min);
        cam.pitch = cam.pitch_min + (cam.pitch_max - cam.pitch_min) * zoom_ratio;

        // Update camera transform based on spherical coordinates
        let x = cam.focus.x + cam.radius * cam.yaw.sin() * cam.pitch.cos();
        let y = cam.focus.y + cam.radius * cam.pitch.sin();
        let z = cam.focus.z + cam.radius * cam.yaw.cos() * cam.pitch.cos();
        transform.translation = Vec3::new(x, y, z);
        transform.look_at(cam.focus, Vec3::Y);
    }
}

fn lerp_snap(from: f32, to: f32, t: f32) -> f32 {
    let result = from + (to - from) * t;
    if (result - to).abs() < 0.0001 {
        to
    } else {
        result
    }
}

fn lerp_snap_vec3(from: Vec3, to: Vec3, t: f32) -> Vec3 {
    let result = from.lerp(to, t);
    if (result - to).length() < 0.0001 {
        to
    } else {
        result
    }
}
