use bevy::prelude::*;

use crate::{cameras::CameraRTS, states::*};

pub struct MainGamePlugin;

impl Plugin for MainGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Gaming), spawn_main_game)
            .add_systems(OnExit(AppState::Gaming), clean_main_menu);
    }
}

#[derive(Component)]
struct MainGame;

fn spawn_main_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera_query: Query<Entity, With<CameraRTS>>,
) {
    commands.spawn((
        MainGame,
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.4, 0.0)),
    ));

    if let Ok(camera_entity) = camera_query.single() {
        commands.entity(camera_entity).insert(AmbientLight {
            color: Color::srgb(0.6, 0.7, 1.0),
            brightness: 500.0,
            ..default()
        });
    }

    let ground_size = 20.0;
    let ground_mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(ground_size / 2.0)));
    let ground_material = materials.add(StandardMaterial::default());

    commands.spawn((
        MainGame,
        Mesh3d(ground_mesh),
        MeshMaterial3d(ground_material),
        Transform::from_xyz(0.0, -0.01, 0.0),
    ));
}

fn clean_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainGame>>,
    camera_query: Query<Entity, With<CameraRTS>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }

    if let Ok(camera_entity) = camera_query.single() {
        commands.entity(camera_entity).remove::<AmbientLight>();
    }
}
