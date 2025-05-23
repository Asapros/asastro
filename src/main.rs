use bevy::prelude::*;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn test_spawn_shapes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let mesh = Mesh2d(meshes.add(Circle::new(50.0)));
    let material_red = MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0)));
    commands.spawn((
        mesh.clone(), material_red.clone(), Transform::from_xyz(-100.0, 0.0, 0.0)
    )
    );
    let material_green = MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 0.0)));
    commands.spawn((
        mesh.clone(), material_green.clone(), Transform::from_xyz(100.0, 0.0, 0.0)
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, test_spawn_shapes)
        .run();
}
