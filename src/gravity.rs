use bevy::log::{debug, info};
use bevy::math::NormedVectorSpace;
use bevy::prelude::*;
use crate::physics::RigidBody;

const G: f32 = 66.743;
pub const DT: f32 = 5.0;

pub(crate) fn tick_gravity(mut bodies: Query<(&Transform, &mut RigidBody)>) {
    let mut iterator = bodies.iter_combinations_mut();
    while let Some([(transform_1, mut body_1), (transform_2, mut body_2)]) = iterator.fetch_next() {
        let translation_1 = transform_1.translation;
        let translation_2 = transform_2.translation;

        let distance_squared = translation_1.distance_squared(translation_2);
        if distance_squared < 10.0 {
            continue;
        }
        let direction_vector_1 = (translation_2 - translation_1).normalize();
        let direction_vector_2 = -direction_vector_1;

        let force = G * body_1.mass * body_2.mass / distance_squared;

        body_1.apply_force(force * direction_vector_1 * DT);
        body_2.apply_force(force * direction_vector_2 * DT);
    }
}
