use bevy::prelude::*;

/// Component that links visual entities to simulation particle indices.
#[derive(Component)]
pub struct ParticleId(pub usize);
