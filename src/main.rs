mod physics;
mod gravity;
mod ui;
mod template;
mod settings;
mod diagnostics;

use bevy::color::palettes::basic::{GRAY, RED, YELLOW};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use crate::diagnostics::DiagnosticsPlugin;
use crate::physics::{PhysicsPlugin, RigidBody};
use crate::settings::SettingsPlugin;
use crate::template::SOLAR_SYSTEM_TEMPLATE;
use crate::ui::UiPlugin;



fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Projection::Orthographic(OrthographicProjection { scale: 0.001, ..OrthographicProjection::default_2d() })));

}

fn test_spawn_planets(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    for body in SOLAR_SYSTEM_TEMPLATE {
        let mesh = Mesh2d(meshes.add(Circle::new(body.radius)));
        let material = MeshMaterial2d(materials.add(body.color));
        commands.spawn((
            mesh, material, Transform::from_xyz(body.aphelion_dist, 0.0, 0.0),
            RigidBody { velocity: Vec3::new(0.0, body.aphelion_speed, 0.0), mass: body.mass }
        ));
    }
    let mesh = Mesh2d(meshes.add(Circle::new(0.0000115)));
    let material = MeshMaterial2d(materials.add(Color::srgb_u8(200, 200, 200)));
    commands.spawn((
        mesh, material, Transform::from_xyz(1.017, 0.00257, 0.0),
        RigidBody { velocity: Vec3::new(0.2151, 6.28, 0.0), mass: 0.0000000363 }
    ));
}
fn test_follow_earth(mut camera: Query<&mut Transform, With<Camera2d>>, planets: Query<(&RigidBody, &Transform), Without<Camera2d>>) {
    let mut camera_transform = camera.single_mut().expect("Camera not found");
    for (body, transform) in planets {
        if body.mass == 0.00000300 {
            camera_transform.translation = transform.translation;
            return;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(SettingsPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DiagnosticsPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, test_spawn_planets)
        .add_systems(Update, test_follow_earth)
        .run();
}
