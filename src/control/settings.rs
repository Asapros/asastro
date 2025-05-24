use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use crate::control::keys::*;

#[derive(Resource)]
pub(crate) struct SimulationSettings {
    pub dt: f32,
    pub stabilized_sps: f32,
    pub pause: bool
}

const DEFAULT_SPS: f32 = 1.01 / 12.0; // A bit over a month
impl Default for SimulationSettings {
    fn default() -> Self {
        Self { dt: 0.0, pause: true, stabilized_sps: DEFAULT_SPS }
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