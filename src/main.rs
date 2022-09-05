use bevy::{
    prelude::*,
    render::mesh::{self, PrimitiveTopology},
};

use bevy_test2::Player;

use bevy_inspector_egui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_plugin(WorldInspectorPlugin::new())
        // .register_type::<Tower>()
        // .add_startup_system(spawn_basic_scene)
        // .add_startup_system(spawn_camera)
        // .add_system(tower_shooting)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn create_triangle() -> Mesh {
    let vertices = [
        ([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0, 2.0, 1.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([2.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
    ];
    let indices = mesh::Indices::U32(vec![0, 2, 1, 0, 3, 2]);
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();

    for (position, normal, uv) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
        uvs.push(*uv);
    }
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    mesh.set_indices(Some(indices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(MaterialMeshBundle {
        mesh: meshes.add(create_triangle()),
        material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
        ..Default::default()
    });

    // commands.spawn().insert(Mesh::from(shape::Box {
    // ..Default::default()
    // }));
}

// #[derive(Component, Reflect, Default)]
// #[reflect(Component)]
// pub struct Tower {
//     shooting_timer: Timer,
// }

// fn tower_shooting(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut towers: Query<&mut Tower>,
//     time: Res<Time>,
// ) {
//     for mut tower in &mut towers {
//         tower.shooting_timer.tick(time.delta());
//         if tower.shooting_timer.just_finished() {
//             let spawn_transform =
//                 Transform::from_xyz(0.0, 0.7, 0.6).with_rotation(Quat::from_rotation_y(-PI / 2.0));

//             commands
//                 .spawn_bundle(PbrBundle {
//                     mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
//                     material: materials.add(Color::rgb(0.87, 0.54, 0.46).into()),
//                     transform: spawn_transform,
//                     ..default()
//                 })
//                 .insert(Name::new("Bullet"));
//         }
//     }
// }

// fn spawn_basic_scene(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     commands
//         .spawn_bundle(PbrBundle {
//             mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
//             material: materials.add(Color::rgb(0.3, 0.5, 0.6).into()),
//             ..default()
//         })
//         .insert(Name::new("Ground"));

//     commands
//         .spawn_bundle(PbrBundle {
//             mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
//             material: materials.add(Color::rgb(1.0, 0.1, 0.1).into()),
//             transform: Transform::from_xyz(0.0, 0.5, 0.0),
//             ..default()
//         })
//         .insert(Name::new("Cube"))
//         .insert(Tower {
//             shooting_timer: Timer::from_seconds(1.0, true),
//         });

//     commands.spawn_bundle(PointLightBundle {
//         point_light: PointLight {
//             intensity: 1500.0,
//             shadows_enabled: true,
//             ..default()
//         },
//         transform: Transform::from_xyz(4.0, 8.0, 4.0),
//         ..default()
//     });
// }
