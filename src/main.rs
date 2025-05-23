mod physics;
mod gravity;
mod ui;

use bevy::color::palettes::basic::{GRAY, RED, YELLOW};
use bevy::prelude::*;
use crate::physics::{PhysicsPlugin, RigidBody};
use crate::ui::UiPlugin;
// Using units:
// distance: AU
// time:     year
// mass:     MO (solar mass)
// This system normalizes gravitational constant G = 4π²

// sun radius: 0.00465
// sun mass: 1

// mercury radius: 0.0000163
// mercury mass: 0.000000166
// mercury aphelion distance: 0.468
// mercury aphelion speed: 8.17


fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Projection::Orthographic(OrthographicProjection { scale: 0.001, ..OrthographicProjection::default_2d() })));

}

fn test_spawn_shapes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    info!("Spawning sun");
    let sun_mesh = Mesh2d(meshes.add(Circle::new(0.00465)));
    let sun_material = MeshMaterial2d(materials.add(Color::from(YELLOW)));
    commands.spawn((
        sun_mesh.clone(), sun_material.clone(), Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody{ velocity: Vec3::new(0.0, 0.0, 0.0), mass: 1.0 }
    )
    );
    info!("Spawning mercury");
    let mercury_mesh = Mesh2d(meshes.add(Circle::new(0.0000163 * 100.0)));
    let mercury_material = MeshMaterial2d(materials.add(Color::from(RED)));
    commands.spawn((
            mercury_mesh.clone(), mercury_material.clone(), Transform::from_xyz(0.468, 0.0, 0.0),
        RigidBody{ velocity: Vec3::new(0.0, 8.17, 0.0), mass: 0.000000166 }
    ));
    info!("done");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugin)
        .add_plugins(UiPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, test_spawn_shapes)
        .run();
}
