use bevy::input::mouse::MouseWheel;
use bevy::math::ops::powf;
use bevy::prelude::*;


#[derive(Resource)]
pub(crate) struct DragInfo {
    drag_start: Option<Vec2>,
}

impl Default for DragInfo {
    fn default() -> Self {
        Self { drag_start: None }
    }
}

const DRAG_BUTTON: MouseButton = MouseButton::Right;
const ZOOM_FACTOR: f32 = 1.2;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Projection::Orthographic(OrthographicProjection { scale: 0.001, ..OrthographicProjection::default_2d() })));
}

fn drag_camera(mut camera: Query<(&Projection, &mut Transform), With<Camera2d>>, buttons: Res<ButtonInput<MouseButton>>, mut camera_info: ResMut<DragInfo>, windows: Query<&Window>) {
    let window = windows.single().expect("Window not found");
    let (projection, mut camera_transform) = camera.single_mut().expect("Camera not found");
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    if buttons.just_pressed(DRAG_BUTTON) {
        camera_info.drag_start = Some(cursor_position);
    }
    if buttons.just_released(DRAG_BUTTON) {
        camera_info.drag_start = None;
        return;
    }
    if !buttons.pressed(DRAG_BUTTON) {
        return;
    }
    
    let cursor_displacement = camera_info.drag_start.unwrap() - cursor_position;
    let Projection::Orthographic(projection) = projection else {
        panic!("Camera is dyslexic (non-orthographic projection set)");
    };
    let camera_displacement = Vec2::new(cursor_displacement.x * projection.scale, cursor_displacement.y * -projection.scale);
    
    camera_transform.translation += camera_displacement.extend(0.0);
    
    camera_info.drag_start = Some(cursor_position);
}

fn zoom_camera(mut camera: Query<(&Camera, &mut Projection, &GlobalTransform, &mut Transform), With<Camera2d>>, mut scroll_events: EventReader<MouseWheel>, windows: Query<&Window>) {
    let steps: f32 = scroll_events.read().map(|event| event.y).sum();
    if steps == 0.0 {
        return;
    }

    let (camera, projection, global_transform, mut transform) = camera.single_mut().expect("Camera not found");

    let window = windows.single().expect("Window not found");
    let Some(cursor_screen) = window.cursor_position() else { return; };
    let Some(cursor_world) = camera.viewport_to_world_2d(global_transform, cursor_screen).ok() else { return; };

    let Projection::Orthographic(projection) = projection.into_inner() else {
        panic!("Camera is dyslexic (non-orthographic projection set)");
    };
    
    let zoom = powf(ZOOM_FACTOR, steps);

    projection.scale /= zoom;

    transform.translation = (cursor_world + (global_transform.translation().truncate() - cursor_world) / zoom).extend(global_transform.translation().z);
    
}

pub(super) struct CameraMovementPlugin;

impl Plugin for CameraMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, drag_camera);
        app.add_systems(Update, zoom_camera);
        app.insert_resource(DragInfo::default());
    }
}