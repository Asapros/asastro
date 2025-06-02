use bevy::app::{App, Plugin, Update};
use crate::control::settings::SimulationSettings;

pub mod settings;
mod keys;
pub(crate) struct SimulationControlPlugin;

impl Plugin for SimulationControlPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationSettings::default());
        app.add_systems(Update, (settings::control_simulation, settings::stabilize_sps));
        app.add_systems(Update, settings::normalize_planets);
    }
}
