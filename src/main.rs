use bevy::prelude::*;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn test_spawn_shape(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let mesh = Mesh2d(meshes.add(Circle::new(50.0)));
    let material = MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0)));
    commands.spawn((
        mesh, material
    ));
    
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, test_spawn_shape)
        .run();
}
