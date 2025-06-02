use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::ecs::bundle::DynamicBundle;
use bevy::ecs::component::ComponentId;
use bevy::prelude::*;
use bevy::text::cosmic_text::ttf_parser::gdef::GlyphClass::Component;
use crate::control::keys::*;
use crate::view::follow::Followable;

#[derive(Resource)]
pub(crate) struct SimulationSettings {
    pub dt: f32,
    pub stabilized_sps: f32,
    pub pause: bool,
    pub normalized: bool
}

const DEFAULT_SPS: f32 = 1.01 / 12.0; // A bit over a month
impl Default for SimulationSettings {
    fn default() -> Self {
        Self { dt: 0.0, pause: true, stabilized_sps: DEFAULT_SPS, normalized: false }
    }
}


pub(super) fn control_simulation(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<SimulationSettings>, time: Res<Time>) {
    if keys.pressed(BUTTON_ACCELERATE_SIMULATION) {
        settings.stabilized_sps *= 1.0 + SPEED_MULTIPLIER * time.delta_secs();
    }
    if keys.pressed(BUTTON_DECELERATE_SIMULATION) {
        settings.stabilized_sps /= 1.0 + SPEED_MULTIPLIER * time.delta_secs();
    }
    if keys.just_pressed(BUTTON_PAUSE_SIMULATION) {
        settings.pause = !settings.pause;
    }
    if keys.just_pressed(BUTTON_REVERSE_SIMULATION) {
        settings.stabilized_sps = -settings.stabilized_sps;
    }
}

pub(super) fn stabilize_sps(mut settings: ResMut<SimulationSettings>, diagnostics: Res<DiagnosticsStore>) {
    let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|d| d.average()) else {
        return;
    };
    settings.dt = settings.stabilized_sps / fps as f32;
}

#[derive(Component)]
pub(crate) struct Normalizable {
    pub(crate) original_mesh: Mesh2d,
    pub(crate) normalized_mesh: Mesh2d,
    pub(crate) original_size: f32
}

pub(crate) const NORMALIZED_SIZE: f32 = 0.05;

pub(super) fn normalize_planets(button: Res<ButtonInput<KeyCode>>, mut settings: ResMut<SimulationSettings>, normalizables: Query<(&Normalizable, Entity, &mut Followable)>, mut commands: Commands) {
    if !button.just_pressed(BUTTON_NORMALIZE_SIZE) { return; }

    settings.normalized = !settings.normalized;

    for (normalizable, entity, mut followable) in normalizables {
        commands.entity(entity).remove::<Mesh2d>();
        commands.entity(entity).insert(match settings.normalized {
            true => normalizable.normalized_mesh.clone(),
            false => normalizable.original_mesh.clone()
        });
        followable.radius = match settings.normalized {
            true => NORMALIZED_SIZE,
            false => normalizable.original_size
        };
    }
}
