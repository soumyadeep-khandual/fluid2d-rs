use bevy::{prelude::*, window::PrimaryWindow};
use rayon::prelude::*;

use crate::{
    components::ParticleId,
    kernels::{poly6_kernel, spiky_kernel_gradient, viscosity_laplacian},
    resources::{BOUNDARY_HEIGHT, BOUNDARY_WIDTH, FluidConfig, FluidSimulation, PARTICLE_RADIUS},
};

/// Handles user input for resetting the simulation.
/// Press 'R' to randomize particle positions.
/// Press 'G' to arrange particles in a grid pattern.
pub fn handle_input(input: Res<ButtonInput<KeyCode>>, mut sim: ResMut<FluidSimulation>) {
    if input.just_pressed(KeyCode::KeyR) {
        sim.reset_random();
    } else if input.just_pressed(KeyCode::KeyG) {
        sim.reset_to_grid();
    }
}

/// Updates the fluid physics simulation using parallel computation.
/// Performs density calculation, pressure computation, force integration, and position updates.
pub fn update_physics_rayon(
    mut sim: ResMut<FluidSimulation>,
    config: Res<FluidConfig>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mouse_btn: Res<ButtonInput<MouseButton>>,
) {
    let dt = 0.002 * config.time_scale;
    if dt <= 0.0 {
        return;
    }

    // Handle mouse interaction
    let mut interaction_pos = Vec2::ZERO;
    let mut interaction_factor = 0.0;

    if let (Ok(window), Ok((camera, camera_transform))) = (q_window.single(), q_camera.single())
        && let Some(cursor_screen_pos) = window.cursor_position()
        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_screen_pos)
    {
        interaction_pos = world_pos;

        if mouse_btn.pressed(MouseButton::Left) {
            interaction_factor = 1.0; // Attract
        } else if mouse_btn.pressed(MouseButton::Right) {
            interaction_factor = -1.0; // Repel
        }
    }

    let sim = &mut *sim;
    let h = config.smoothing_radius;
    let h_sq = h * h;
    let target_density = config.target_density;
    let pressure_k = config.pressure_multiplier;
    let viscosity_mu = config.viscosity_strength;
    let gravity = config.gravity;
    let interact_rad = config.mouse_radius;
    let interact_str = config.mouse_strength;

    // Rebuild spatial grid for neighbor searches
    sim.grid_map.par_iter_mut().for_each(|cell| cell.clear());
    let grid_w = sim.grid_width_cells;
    let cell_size = sim.grid_cell_size;
    let off_x = sim.grid_offset_x;
    let off_y = sim.grid_offset_y;

    for (i, pos) in sim.positions.iter().enumerate() {
        let gx = ((pos.x + off_x) / cell_size) as usize;
        let gy = ((pos.y + off_y) / cell_size) as usize;
        let idx = (gy * grid_w + gx).clamp(0, sim.grid_map.len() - 1);
        sim.grid_map[idx].push(i);
    }
    let positions = &sim.positions;
    let grid = &sim.grid_map;

    // Calculate density and pressure for each particle
    sim.densities
        .par_iter_mut()
        .zip(&mut sim.pressures)
        .enumerate()
        .for_each(|(i, (density_out, pressure_out))| {
            let pos = positions[i];
            let mut d = 0.0;
            let gx = ((pos.x + off_x) / cell_size) as usize;
            let gy = ((pos.y + off_y) / cell_size) as usize;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let cx = (gx as isize + dx) as usize;
                    let cy = (gy as isize + dy) as usize;
                    if cx >= sim.grid_width_cells || cy >= sim.grid_height_cells {
                        continue;
                    }
                    if let Some(cell) = grid.get(cy * grid_w + cx) {
                        for &j in cell {
                            let dist_sq = pos.distance_squared(positions[j]);
                            if dist_sq < h_sq {
                                d += config.particle_mass * poly6_kernel(dist_sq, h);
                            }
                        }
                    }
                }
            }
            *density_out = d;
            *pressure_out = pressure_k * (d - target_density);
        });

    let densities = &sim.densities;
    let pressures = &sim.pressures;
    let velocities = &sim.velocities;

    // Calculate forces (pressure, viscosity, interaction)
    sim.forces
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, force_out)| {
            let pos = positions[i];
            let dens = densities[i];
            let press = pressures[i];
            let vel = velocities[i];

            let mut f_pressure = Vec2::ZERO;
            let mut f_viscosity = Vec2::ZERO;
            let gx = ((pos.x + off_x) / cell_size) as usize;
            let gy = ((pos.y + off_y) / cell_size) as usize;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let cx = (gx as isize + dx) as usize;
                    let cy = (gy as isize + dy) as usize;
                    if cx >= sim.grid_width_cells || cy >= sim.grid_height_cells {
                        continue;
                    }

                    if let Some(cell) = grid.get(cy * grid_w + cx) {
                        for &j in cell {
                            if i == j {
                                continue;
                            }
                            let other_pos = positions[j];
                            let dist = pos.distance(other_pos);

                            if dist < h && dist > 0.0001 {
                                let dir = (other_pos - pos) / dist;
                                let safe_dens = densities[j].max(0.0001);

                                let slope = spiky_kernel_gradient(dist, h);
                                let pressure_term =
                                    (press / dens / dens) + (pressures[j] / safe_dens / safe_dens);
                                f_pressure += -config.particle_mass
                                    * config.particle_mass
                                    * pressure_term
                                    * slope
                                    * dir;

                                let vel_diff = velocities[j] - vel;
                                let laplacian = viscosity_laplacian(dist, h);
                                f_viscosity += vel_diff
                                    * viscosity_mu
                                    * laplacian
                                    * (1.0 / safe_dens)
                                    * config.particle_mass;
                            }
                        }
                    }
                }
            }

            let mut f_interaction = Vec2::ZERO;
            if interaction_factor != 0.0 {
                let to_mouse = interaction_pos - pos;
                let dist = to_mouse.length();
                if dist < interact_rad && dist > 0.001 {
                    let dir = to_mouse / dist;
                    let strength = interact_str * (1.0 - dist / interact_rad);
                    f_interaction = dir * strength * interaction_factor;
                }
            }

            *force_out = f_pressure + f_viscosity + (gravity * dens) + f_interaction;
        });
    sim.positions
        .par_iter_mut()
        .zip(&mut sim.velocities)
        .zip(&sim.forces)
        .zip(&sim.densities)
        .for_each(|(((pos, vel), force), dens)| {
            let acceleration = *force / dens.max(0.0001);
            *vel += acceleration * dt;
            *vel *= 0.99; // Numerical damping
            *pos += *vel * dt;

            let w = BOUNDARY_WIDTH / 2.0 - PARTICLE_RADIUS;
            let hh = BOUNDARY_HEIGHT / 2.0 - PARTICLE_RADIUS;
            let restitution = config.boundary_damping;

            if pos.x < -w {
                pos.x = -w;
                vel.x *= -restitution;
            } else if pos.x > w {
                pos.x = w;
                vel.x *= -restitution;
            }

            if pos.y < -hh {
                pos.y = -hh;
                vel.y = vel.y.max(0.0) * restitution;
            } else if pos.y > hh {
                pos.y = hh;
                vel.y *= -restitution;
            }
        });
}

/// Sets up the initial scene with particle entities and camera.
pub fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sim: Res<FluidSimulation>,
) {
    commands.spawn(Camera2d);
    let tex = asset_server.load("circle.png");
    let bundles: Vec<_> = (0..sim.positions.len())
        .map(|i| {
            (
                Sprite {
                    image: tex.clone(),
                    custom_size: Some(Vec2::splat(PARTICLE_RADIUS * 2.5)),
                    color: Color::srgb(0.2, 0.5, 1.0),
                    ..default()
                },
                Transform::from_translation(sim.positions[i].extend(0.0)),
                ParticleId(i),
            )
        })
        .collect();
    commands.spawn_batch(bundles);
}

/// Synchronizes particle visual representation with simulation state.
/// Updates positions and colors particles based on velocity.
pub fn sync_rendering(
    sim: Res<FluidSimulation>,
    mut query: Query<(&mut Transform, &mut Sprite, &ParticleId)>,
) {
    let max_sq = 400.0f32.powi(2);
    query.par_iter_mut().for_each(|(mut t, mut s, pid)| {
        let i = pid.0;
        if let Some(pos) = sim.positions.get(i) {
            t.translation.x = pos.x;
            t.translation.y = pos.y;
            t.translation.z = (i % 100) as f32 * 0.001;
        }
        if let Some(vel) = sim.velocities.get(i) {
            let n = (vel.length_squared() / max_sq).clamp(0.0, 1.0).sqrt();
            s.color = Color::mix(&Color::srgb(0.1, 0.2, 0.9), &Color::srgb(1.0, 1.0, 1.0), n); // Velocity-based coloring
        }
    });
}
