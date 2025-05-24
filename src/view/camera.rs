use bevy::input::mouse::MouseWheel;
use bevy::math::ops::powf;
use bevy::prelude::*;
use crate::physics::rigid_body::{tick_velocity, RigidBody};

#[derive(Resource)]
struct DragInfo {
    cursor_start: Option<Vec2>,
    camera_start: Option<Vec2>
}

const DRAG_BUTTON: MouseButton = MouseButton::Left;
const ZOOM_FACTOR: f32 = 1.2;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Projection::Orthographic(OrthographicProjection { scale: 0.001, ..OrthographicProjection::default_2d() })));
}

fn drag_camera(mut camera: Query<(&Camera, &GlobalTransform, &mut Transform), With<Camera2d>>, buttons: Res<ButtonInput<MouseButton>>, mut drag: ResMut<DragInfo>, windows: Query<&Window>) {
    let window = windows.single().expect("Window not found");
    let (camera, global_transform, mut camera_transform) = camera.single_mut().expect("Camera not found");
    let camera_position = global_transform.translation().truncate();
    let Some(cursor_position) = window.cursor_position().and_then(|cursor| camera.viewport_to_world_2d(global_transform, cursor).ok()) else {
        return;
    };

    if buttons.just_pressed(DRAG_BUTTON) {
        drag.cursor_start = Some(cursor_position);
        drag.camera_start = Some(camera_position);
    }
    if buttons.just_released(DRAG_BUTTON) {
        drag.cursor_start = None;
        drag.camera_start = None;
    }
    if !buttons.pressed(DRAG_BUTTON) {
        return;
    }
    let cursor_displacement = drag.cursor_start.unwrap() - cursor_position;
    let new_camera_position = camera_position + cursor_displacement;

    camera_transform.translation.x = new_camera_position.x;
    camera_transform.translation.y = new_camera_position.y;
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

fn test_follow_earth(mut camera: Query<&mut Transform, With<Camera2d>>, planets: Query<(&RigidBody, &Transform), Without<Camera2d>>) {
    let mut camera_transform = camera.single_mut().expect("Camera not found");
    for (body, transform) in planets {
        if body.mass == 0.00000300 {
            camera_transform.translation = transform.translation;
            return;
        }
    }
}
pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, drag_camera);
        app.add_systems(Update, zoom_camera);
        app.add_systems(Update, test_follow_earth.after(drag_camera).after(zoom_camera).after(tick_velocity));
        app.insert_resource(DragInfo { cursor_start: None, camera_start: None });
    }
}