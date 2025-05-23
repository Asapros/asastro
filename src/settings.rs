use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::prelude::KeyCode::KeyA;

#[derive(Resource)]
pub struct SimulationSettings {
    pub dt: f32,
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
        settings.dt *= 1.0 + SPEED_MULTIPLIER * time.delta_secs();
    }
    if keys.pressed(BUTTON_DECELERATE_SIMULATION) {
        settings.dt /= 1.0 + SPEED_MULTIPLIER * time.delta_secs();
    }
    if keys.just_pressed(BUTTON_PAUSE_SIMULATION) {
        settings.pause = !settings.pause;
    }
    if keys.just_pressed(BUTTON_REVERSE_SIMULATION) {
        settings.dt = -settings.dt;
    }
}

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationSettings { dt: 0.0004, pause: true });
        app.add_systems(Update, control_simulation);
    }
}