use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Recantgular Base
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1000.0, 1000.0, 100.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(1000.0 / 2.0, 1000.0 / 2.0, 100.0 / 2.0),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4000.0, 8000.0, 4000.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-484.5, 854.0, 1708.5).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
