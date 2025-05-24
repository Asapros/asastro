use bevy::prelude::*;

pub mod rigid_body;
mod gravity;


pub(crate) struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (gravity::tick_gravity, rigid_body::tick_velocity).chain());
    }
}
