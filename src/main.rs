mod physics;
mod gravity;
mod ui;
mod template;
mod settings;

use bevy::color::palettes::basic::{GRAY, RED, YELLOW};
use bevy::prelude::*;
use crate::physics::{PhysicsPlugin, RigidBody};
use crate::settings::SettingsPlugin;
use crate::template::SOLAR_SYSTEM_TEMPLATE;
use crate::ui::UiPlugin;



fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Projection::Orthographic(OrthographicProjection { scale: 0.001, ..OrthographicProjection::default_2d() })));

}

fn test_spawn_shapes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    for body in SOLAR_SYSTEM_TEMPLATE {
        info!("Spawning {}", body.name);
        let mesh = Mesh2d(meshes.add(Circle::new(0.15)));
        let material = MeshMaterial2d(materials.add(body.color));
        commands.spawn((
            mesh, material, Transform::from_xyz(body.aphelion_dist, 0.0, 0.0),
            RigidBody { velocity: Vec3::new(0.0, body.aphelion_speed, 0.0), mass: body.mass }
        ));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(SettingsPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, test_spawn_shapes)
        .run();
}
