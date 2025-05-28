use bevy::input::mouse::MouseWheel;
use bevy::math::ops::powf;
use bevy::prelude::*;
use crate::physics::rigid_body::{tick_velocity, RigidBody};

#[derive(Copy, Clone)]
pub(crate) struct FollowInfo {
    pub(crate) followed: Entity,
    pub(crate) previous_position: Vec3
}

#[derive(Resource)]
pub(crate) struct ViewInfo {
    drag_start: Option<Vec2>,
    pub(crate) follow: Option<FollowInfo>
}

const DRAG_BUTTON: MouseButton = MouseButton::Right;
const ZOOM_FACTOR: f32 = 1.2;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Projection::Orthographic(OrthographicProjection { scale: 0.001, ..OrthographicProjection::default_2d() })));
}

fn drag_camera(mut camera: Query<(&Projection, &mut Transform), With<Camera2d>>, buttons: Res<ButtonInput<MouseButton>>, mut camera_info: ResMut<ViewInfo>, windows: Query<&Window>) {
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

fn test_follow_mercury(mut camera: Query<&mut Transform, With<Camera2d>>, planets: Query<(Entity, &Transform), Without<Camera2d>>, mut view_info: ResMut<ViewInfo>) {
    let Some(following) = &mut view_info.into_inner().follow else {
        return;
    };
    let mut camera_transform = camera.single_mut().expect("Camera not found");
    for (entity, transform) in planets {
        if entity != following.followed { continue };
        let delta = transform.translation - following.previous_position;
        camera_transform.translation += delta;
        following.previous_position = transform.translation;
    }
}
pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, drag_camera);
        app.add_systems(Update, zoom_camera);
        app.add_systems(Update, test_follow_mercury.after(tick_velocity));
        app.insert_resource(ViewInfo { drag_start: None, follow: None });
    }
}