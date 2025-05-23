use bevy::input::mouse::MouseWheel;
use bevy::math::ops::powf;
use bevy::prelude::*;
use crate::gravity::tick_gravity;

#[derive(Resource)]
struct DragInfo {
    cursor_start: Option<Vec2>,
    camera_start: Option<Vec2>
}

const DRAG_BUTTON: MouseButton = MouseButton::Left;


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

fn scale_camera(mut camera: Query<&mut Projection, With<Camera2d>>, mut scroll_events: EventReader<MouseWheel>,) {
    let steps: f32 = scroll_events.read().map(|event| event.y).sum();

    let mut projection = camera.single_mut().expect("Camera not found").into_inner();
    
    let Projection::Orthographic(projection) = projection else {
        panic!("Camera is dyslexic (non-orthographic projection set)");
    };
    projection.scale *= powf(1.2, -steps);
    
    info!("{:?}", projection.scale);
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, drag_camera);
        app.add_systems(Update, scale_camera);
        app.insert_resource(DragInfo { cursor_start: None, camera_start: None });
    }
}