mod physics;
mod view;
mod control;
mod initialization;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use crate::control::SimulationControlPlugin;
use crate::initialization::SimulationInitializerPlugin;
use crate::physics::PhysicsPlugin;
use crate::physics::rigid_body::RigidBody;
use crate::view::UniverseViewPlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(SimulationInitializerPlugin)
        .add_plugins(SimulationControlPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(UniverseViewPlugin)
        .run();
}
