use bevy::asset::Assets;
use bevy::color::Color;
use bevy::math::Vec3;
use bevy::prelude::*;
use crate::control::settings::{Normalizable, NORMALIZED_SIZE};
use crate::initialization::template::SOLAR_SYSTEM_TEMPLATE;
use crate::physics::rigid_body::RigidBody;
use crate::view::follow::{FollowInfo, Followable};

pub(super) fn spawn_solar_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    for (index, body) in SOLAR_SYSTEM_TEMPLATE.iter().enumerate() {
        // let mesh = Mesh2d(meshes.add(Circle::new(body.radius)));
        let mesh = Mesh2d(meshes.add(Circle::new(body.radius)));
        let normalized_mesh = Mesh2d(meshes.add(Circle::new(NORMALIZED_SIZE)));
        let material = MeshMaterial2d(materials.add(body.color));
        let transform = Transform::from_xyz(body.aphelion_dist, 0.0, 0.0);
        let entity = commands.spawn((
            mesh.clone(), material, transform,
            RigidBody { velocity: Vec3::new(0.0, body.aphelion_speed, 0.0), mass: body.mass },
            Followable { radius: body.radius, name: body.name.to_string(), bind: index.try_into().ok() },
            Normalizable { normalized_mesh: normalized_mesh, original_mesh: mesh.clone(), original_size: body.radius }
        ));
    }
    // Spawn moon
    let mesh = Mesh2d(meshes.add(Circle::new(0.0000115)));
    let material = MeshMaterial2d(materials.add(Color::srgb_u8(200, 200, 200)));
    commands.spawn((
        mesh, material, Transform::from_xyz(1.017, 0.00257, 0.0),
        RigidBody { velocity: Vec3::new(0.2151, 6.28, 0.0), mass: 0.0000000363 },
    ));
}
