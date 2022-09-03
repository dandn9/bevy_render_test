use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "Bevy Tower Defense".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .register_type::<Tower>()
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_camera)
        .add_system(tower_shooting)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Tower {
    shooting_timer: Timer,
}

fn tower_shooting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut towers: Query<&mut Tower>,
    time: Res<Time>,
) {
    for mut tower in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let spawn_transform =
                Transform::from_xyz(0.0, 0.7, 0.6).with_rotation(Quat::from_rotation_y(-PI / 2.0));

            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                    material: materials.add(Color::rgb(0.87, 0.44, 0.42).into()),
                    transform: spawn_transform,
                    ..default()
                })
                .insert(Name::new("Bullet"));
        }
    }
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.6).into()),
            ..default()
        })
        .insert(Name::new("Ground"));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(1.0, 0.1, 0.1).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Name::new("Cube"))
        .insert(Tower {
            shooting_timer: Timer::from_seconds(1.0, true),
        });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
