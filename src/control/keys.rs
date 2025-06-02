use bevy::prelude::KeyCode;

pub(super) const BUTTON_ACCELERATE_SIMULATION: KeyCode = KeyCode::Period;
pub(super) const BUTTON_DECELERATE_SIMULATION: KeyCode = KeyCode::Comma;
pub(super) const BUTTON_PAUSE_SIMULATION: KeyCode = KeyCode::Space;
pub(super) const BUTTON_REVERSE_SIMULATION: KeyCode = KeyCode::Semicolon;
pub(super) const SPEED_MULTIPLIER: f32 = 1.0;
pub(super) const BUTTON_NORMALIZE_SIZE: KeyCode = KeyCode::KeyN;