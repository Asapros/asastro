use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;

mod template;
mod spawn;

pub(crate) struct SimulationInitializerPlugin;

impl Plugin for SimulationInitializerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn::spawn_solar_system);
    }
}
