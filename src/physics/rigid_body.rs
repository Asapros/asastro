use bevy::prelude::*;
use crate::control::settings::SimulationSettings;

#[derive(Component)]
pub(crate) struct RigidBody {
    pub mass: f32,
    pub velocity: Vec3
}

impl Default for RigidBody {
    fn default() -> Self {
        Self { mass: 1.0, velocity: Vec3::ZERO }
    }
}

impl RigidBody {
    pub(crate) fn apply_force(&mut self, force: Vec3, period: f32) {
        self.velocity += force / self.mass * period;
    }
}

pub(crate) fn tick_velocity(mut bodies: Query<(&mut Transform, &RigidBody)>, settings: Res<SimulationSettings>) {
    if settings.pause { return }
    
    for (mut transform, body) in &mut bodies {
        transform.translation += body.velocity * settings.dt;
    }
}


