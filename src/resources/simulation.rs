use bevy::prelude::*;
use rand::{rng, Rng};

use super::config::{BOUNDARY_HEIGHT, BOUNDARY_WIDTH, PARTICLE_COUNT};

/// Core simulation data structure containing all particle state.
/// Uses pre-allocated vectors for performance and memory efficiency.
#[derive(Resource)]
pub struct FluidSimulation {
    pub positions: Vec<Vec2>,
    pub velocities: Vec<Vec2>,
    pub forces: Vec<Vec2>,
    pub densities: Vec<f32>,
    pub pressures: Vec<f32>,
    pub grid_map: Vec<Vec<usize>>,
    pub grid_cell_size: f32,
    pub grid_width_cells: usize,
    pub grid_height_cells: usize,
    pub grid_offset_x: f32,
    pub grid_offset_y: f32,
}

impl FluidSimulation {
    /// Creates a new fluid simulation with pre-allocated data structures.
    pub fn new() -> Self {
        let max_h = 25.0;
        let grid_w_cells = (BOUNDARY_WIDTH / max_h).ceil() as usize + 4;
        let grid_h_cells = (BOUNDARY_HEIGHT / max_h).ceil() as usize + 4;

        let mut sim = Self {
            positions: Vec::with_capacity(PARTICLE_COUNT),
            velocities: Vec::with_capacity(PARTICLE_COUNT),
            forces: vec![Vec2::ZERO; PARTICLE_COUNT],
            densities: vec![0.0; PARTICLE_COUNT],
            pressures: vec![0.0; PARTICLE_COUNT],
            grid_map: vec![Vec::with_capacity(20); grid_w_cells * grid_h_cells],
            grid_cell_size: max_h,
            grid_width_cells: grid_w_cells,
            grid_height_cells: grid_h_cells,
            grid_offset_x: BOUNDARY_WIDTH / 2.0 + max_h * 2.0,
            grid_offset_y: BOUNDARY_HEIGHT / 2.0 + max_h * 2.0,
        };
        sim.reset_random();
        sim
    }

    /// Resets the simulation with random particle positions.
    pub fn reset_random(&mut self) {
        self.positions.clear();
        self.velocities.clear();
        let mut rng = rng();
        let w = BOUNDARY_WIDTH / 2.0 - 20.0;
        let h = BOUNDARY_HEIGHT / 2.0 - 20.0;
        for _ in 0..PARTICLE_COUNT {
            self.positions
                .push(Vec2::new(rng.random_range(-w..w), rng.random_range(-h..h)));
            self.velocities.push(Vec2::ZERO);
        }
        self.forces.fill(Vec2::ZERO);
        self.densities.fill(0.0);
        self.pressures.fill(0.0);
    }

    /// Resets the simulation with particles arranged in a grid pattern.
    /// Calculates optimal grid dimensions and spacing to fit within 70% of boundary dimensions.
    pub fn reset_to_grid(&mut self) {
        self.positions.clear();
        self.velocities.clear();

        // Calculate roughly square grid
        let grid_size = (PARTICLE_COUNT as f32).sqrt().ceil() as usize;
        let cols = grid_size;
        let rows = PARTICLE_COUNT.div_ceil(cols);

        let available_width = BOUNDARY_WIDTH * 0.7;
        let available_height = BOUNDARY_HEIGHT * 0.7;

        let spacing_x = if cols > 1 {
            available_width / (cols - 1) as f32
        } else {
            available_width
        };
        let spacing_y = if rows > 1 {
            available_height / (rows - 1) as f32
        } else {
            available_height
        };

        let spacing = spacing_x.min(spacing_y).clamp(12.0, 20.0);
        let total_width = (cols - 1) as f32 * spacing;
        let total_height = (rows - 1) as f32 * spacing;

        let start_x = -total_width / 2.0;
        let start_y = -total_height / 2.0;

        for row in 0..rows {
            for col in 0..cols {
                let particle_index = row * cols + col;
                if particle_index >= PARTICLE_COUNT {
                    break;
                }

                let x = start_x + col as f32 * spacing;
                let y = start_y + row as f32 * spacing;

                self.positions.push(Vec2::new(x, y));
                self.velocities.push(Vec2::ZERO);
            }
        }

        self.forces.fill(Vec2::ZERO);
        self.densities.fill(0.0);
        self.pressures.fill(0.0);
    }
}
