use bevy::prelude::*;

/// Number of particles in the simulation.
pub const PARTICLE_COUNT: usize = 4_000;
/// Visual radius of each particle.
pub const PARTICLE_RADIUS: f32 = 2.0;
/// Width of the simulation boundary.
pub const BOUNDARY_WIDTH: f32 = 1280.0;
/// Height of the simulation boundary.
pub const BOUNDARY_HEIGHT: f32 = 720.0;

/// Configuration parameters for the fluid simulation.
/// This resource is automatically exposed to the Bevy Inspector for runtime tweaking.
#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct FluidConfig {
    /// Smoothing radius for SPH kernels (h).
    pub smoothing_radius: f32,
    /// Mass of each particle.
    pub particle_mass: f32,
    /// Target rest density for pressure calculations.
    pub target_density: f32,
    /// Gas constant multiplier for pressure forces (K).
    pub pressure_multiplier: f32,
    /// Viscosity coefficient for viscous forces (μ).
    pub viscosity_strength: f32,
    /// Gravitational acceleration vector.
    pub gravity: Vec2,
    /// Time step scaling factor.
    pub time_scale: f32,
    /// Velocity damping factor for boundary collisions (0.0 = no bounce, 1.0 = perfect bounce).
    pub boundary_damping: f32,
    /// Radius of mouse interaction influence.
    pub mouse_radius: f32,
    /// Strength of mouse interaction forces.
    pub mouse_strength: f32,
}

impl Default for FluidConfig {
    fn default() -> Self {
        Self {
            smoothing_radius: 20.0,
            particle_mass: 1.0,
            target_density: 0.01,
            pressure_multiplier: 200.0,
            viscosity_strength: 50.0,
            gravity: Vec2::new(0.0, -100.0),
            time_scale: 10.0,
            boundary_damping: 0.4,
            mouse_radius: 200.0,
            mouse_strength: 10.0,
        }
    }
}
