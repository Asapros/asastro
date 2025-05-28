use bevy::app::{App, Plugin, Startup, Update};
use bevy::math::Vec3;
use bevy::prelude::*;
use crate::physics::rigid_body::tick_velocity;

const SELECT_BUTTON: MouseButton = MouseButton::Left;
#[derive(Resource)]
pub(crate) struct FollowInfo {
    pub(crate) entity: Option<Entity>,
    pub(crate) previous_position: Option<Vec3>,
    pub(crate) name: Option<String>
}

#[derive(Component)]
pub(crate) struct Followable {
    pub(crate) radius: f32,
    pub(crate) name: String
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
pub(super) struct CameraFollowPlugin;
impl Plugin for CameraFollowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FollowInfo::default());
        app.add_systems(Update, follow_entity.after(tick_velocity));
        app.add_systems(Update, select_followable);
    }
}
