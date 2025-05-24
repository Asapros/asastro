use bevy::asset::Assets;
use bevy::color::Color;
use bevy::math::Vec3;
use bevy::prelude::{Circle, ColorMaterial, Commands, Mesh, Mesh2d, MeshMaterial2d, ResMut, Transform};
use crate::initialization::template::SOLAR_SYSTEM_TEMPLATE;
use crate::physics::rigid_body::RigidBody;

pub(super) fn spawn_solar_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    for body in SOLAR_SYSTEM_TEMPLATE {
        let mesh = Mesh2d(meshes.add(Circle::new(body.radius)));
        let material = MeshMaterial2d(materials.add(body.color));
        commands.spawn((
            mesh, material, Transform::from_xyz(body.aphelion_dist, 0.0, 0.0),
            RigidBody { velocity: Vec3::new(0.0, body.aphelion_speed, 0.0), mass: body.mass }
        ));
    }
    // Spawn moon
    let mesh = Mesh2d(meshes.add(Circle::new(0.0000115)));
    let material = MeshMaterial2d(materials.add(Color::srgb_u8(200, 200, 200)));
    commands.spawn((
        mesh, material, Transform::from_xyz(1.017, 0.00257, 0.0),
        RigidBody { velocity: Vec3::new(0.2151, 6.28, 0.0), mass: 0.0000000363 }
    ));
}
