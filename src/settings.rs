use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::prelude::KeyCode::KeyA;
use crate::diagnostics::SOLAR_YEAR_DAYS;

#[derive(Resource)]
pub struct SimulationSettings {
    pub dt: f32,
    pub stabilized_sps: f32,
    pub pause: bool
}

const BUTTON_ACCELERATE_SIMULATION: KeyCode = KeyCode::Period;
const BUTTON_DECELERATE_SIMULATION: KeyCode = KeyCode::Comma;
const BUTTON_PAUSE_SIMULATION: KeyCode = KeyCode::Space;
const BUTTON_REVERSE_SIMULATION: KeyCode = KeyCode::Semicolon;
const SPEED_MULTIPLIER: f32 = 1.0;
pub struct SettingsPlugin;

fn control_simulation(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<SimulationSettings>, time: Res<Time>) {
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

fn stabilize_sps(mut settings: ResMut<SimulationSettings>, diagnostics: Res<DiagnosticsStore>) {
    let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|d| d.average()) else {
        return;
    };
    settings.dt = settings.stabilized_sps / fps as f32;
}

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationSettings { dt: 0.0, pause: true, stabilized_sps: 1.01 / 12.0 });
        app.add_systems(Update, (control_simulation, stabilize_sps));
    }
}