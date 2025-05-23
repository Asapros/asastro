mod physics;
mod gravity;

use bevy::prelude::*;
use crate::physics::{PhysicsPlugin, RigidBody};

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Projection::Orthographic(OrthographicProjection { scale: 2.0, ..OrthographicProjection::default_2d() })));
    
}

fn test_spawn_shapes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let mesh = Mesh2d(meshes.add(Circle::new(50.0)));
    let material_red = MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0)));
    commands.spawn((
        mesh.clone(), material_red.clone(), Transform::from_xyz(-100.0, 0.0, 0.0),
        RigidBody{ velocity: Vec3::new(0.0, 0.5, 0.0), mass: 0.001 }
    )
    );
    let material_green = MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 0.0)));
    commands.spawn((
        mesh.clone(), material_green.clone(), Transform::from_xyz(100.0, 0.0, 0.0),
        RigidBody{ velocity: Vec3::new(0.0, 0.0, 0.0), ..default() }
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, test_spawn_shapes)
        .run();
}
