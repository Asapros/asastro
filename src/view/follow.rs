use bevy::app::{App, Plugin, Startup, Update};
use bevy::input::keyboard::Key;
use bevy::math::Vec3;
use bevy::prelude::*;
use crate::physics::rigid_body::tick_velocity;

const SELECT_BUTTON: MouseButton = MouseButton::Left;
const FOCUS_BUTTON: KeyCode = KeyCode::KeyZ;

#[derive(Resource)]
pub(crate) struct FollowInfo {
    pub(crate) entity: Option<Entity>,
    pub(crate) previous_position: Option<Vec3>,
    pub(crate) name: Option<String>
}

#[derive(Component)]
pub(crate) struct Followable {
    pub(crate) radius: f32,
    pub(crate) name: String,
    pub(crate) bind: Option<u8>
}

impl Default for FollowInfo {
    fn default() -> Self {
        Self { entity: None, previous_position: None, name: None }
    }
}

fn follow_entity(mut camera: Query<&mut Transform, With<Camera2d>>, planets: Query<(Entity, &Transform), Without<Camera2d>>, mut follow_info: ResMut<FollowInfo>) {
    let Some(followed) = follow_info.entity else {
        return;
    };
    let previous_position = follow_info.previous_position.unwrap();

    let mut camera_transform = camera.single_mut().expect("Camera not found");
    for (entity, transform) in planets {
        if entity != followed { continue };
        let delta = transform.translation - previous_position;
        camera_transform.translation += delta;
        follow_info.previous_position = Some(transform.translation);
    }
}

fn select_followable(followables: Query<(Entity, &Transform, &Followable)>, mut camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>, windows: Query<&Window>, button: Res<ButtonInput<MouseButton>>, mut follow_info: ResMut<FollowInfo>) {
    if !button.just_pressed(SELECT_BUTTON) {
        return;
    }
    let (camera, camera_transform) = camera.single_mut().expect("Camera not found");
    let window = windows.single().expect("Window not found");
    let Some(cursor_screen) = window.cursor_position() else { return; };
    let Some(cursor_world) = camera.viewport_to_world_2d(camera_transform, cursor_screen).ok() else { return; };

    for (entity, transform, followable) in followables {
        if transform.translation.truncate().distance(cursor_world) > followable.radius { continue }
        follow_info.entity = Some(entity);
        follow_info.previous_position = Some(transform.translation);
        follow_info.name = Some(followable.name.clone());
        return;
    }
    follow_info.entity = None;
    follow_info.previous_position = None;
    follow_info.name = None;
}

fn focus_selected(follow_info: Res<FollowInfo>, mut camera: Query<(&mut Transform, &mut Projection), With<Camera2d>>, button: Res<ButtonInput<KeyCode>>, followables: Query<(Entity, &Transform, &Followable), Without<Camera2d>>) {
    if !button.just_pressed(FOCUS_BUTTON) {
        return;
    }
    let (mut camera_transform, mut projection) = camera.single_mut().expect("Camera not found");
    let Projection::Orthographic(projection) = projection.into_inner() else {
        panic!("Camera is dyslexic (non-orthographic projection set)");
    };
    let Some(followed_entity) = follow_info.entity else {
        camera_transform.translation = Vec3::splat(0.0);
        projection.scale = 0.001;
        return;
    };
    for (entity, transform, followable) in followables {
        if entity != followed_entity { continue; }
        camera_transform.translation = transform.translation;
        projection.scale = followable.radius / 100.0;
    }
    
}

const FOLLOW_KEYS: [KeyCode; 10] = [
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::Digit4,
    KeyCode::Digit5,
    KeyCode::Digit6,
    KeyCode::Digit7,
    KeyCode::Digit8,
    KeyCode::Digit9,
    KeyCode::Digit0
];

fn follow_binds(followables: Query<(Entity, &Transform, &Followable)>, button: Res<ButtonInput<KeyCode>>, mut follow_info: ResMut<FollowInfo>) {
    for (index, key) in FOLLOW_KEYS.iter().enumerate() {
        if !button.just_pressed(key.clone()) { continue }
        for (entity, transform, followable) in followables {
            if followable.bind != Some(index as u8) { continue; }
            follow_info.entity = Some(entity);
            follow_info.previous_position = Some(transform.translation);
            follow_info.name = Some(followable.name.clone());
            break;
        }
    }
}
pub(super) struct CameraFollowPlugin;
impl Plugin for CameraFollowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FollowInfo::default());
        app.add_systems(Update, follow_entity.after(tick_velocity));
        app.add_systems(Update, select_followable);
        app.add_systems(Update, focus_selected);
        app.add_systems(Update, follow_binds);
    }
}
