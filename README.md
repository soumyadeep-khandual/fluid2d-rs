# Fluid2D

A real-time 2D fluid simulation using Smoothed-Particle Hydrodynamics (SPH) implemented in Rust with the Bevy game engine.

![Fluid Simulation Demo](https://img.shields.io/badge/SPH-Fluid_Simulation-blue) ![Rust](https://img.shields.io/badge/Rust-2024-black) ![Bevy](https://img.shields.io/badge/Bevy-0.17-orange)

## ✨ Features

- **Real-time SPH Simulation**: Fluid dynamics using Smoothed-Particle Hydrodynamics
- **Interactive Controls**: Mouse interaction for attracting/repelling particles
- **Real-time Parameter Tuning**: Bevy Inspector integration for live parameter adjustment
- **High Performance**: Parallel computation using Rayon, optimized for modern CPUs
- **Boundary Handling**: Collision detection with configurable damping
- **Visual Feedback**: Velocity-based particle coloring for flow visualization

## 🎮 Controls

| Key | Action |
|-----|--------|
| `R` | Random particle distribution |
| `G` | Grid particle arrangement |
| `Mouse Left` | Attract particles |
| `Mouse Right` | Repel particles |

## 🚀 Installation & Running

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd fluid2d
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Run the simulation**:
   ```bash
   cargo run --release
   ```

## ⚙️ Configuration

The simulation parameters can be adjusted in real-time using the **Bevy Inspector**:

### Core SPH Parameters
- **Smoothing Radius** (`smoothing_radius`): Kernel support radius (default: 20.0)
- **Particle Mass** (`particle_mass`): Mass of each particle (default: 1.0)
- **Target Density** (`target_density`): Rest density for pressure calculations (default: 0.01)

### Physical Properties
- **Pressure Multiplier** (`pressure_multiplier`): Gas constant for incompressibility (default: 200.0)
- **Viscosity Strength** (`viscosity_strength`): Fluid viscosity coefficient (default: 50.0)
- **Gravity** (`gravity`): Gravitational acceleration vector (default: (0.0, -100.0))

### Simulation Control
- **Time Scale** (`time_scale`): Time step multiplier (default: 10.0)
- **Boundary Damping** (`boundary_damping`): Wall collision damping (0.0-1.0, default: 0.4)

### User Interaction
- **Mouse Radius** (`mouse_radius`): Interaction influence radius (default: 50.0)
- **Mouse Strength** (`mouse_strength`): Interaction force strength (default: 200.0)


### Performance Optimizations
- **Spatial Grid**: O(N) neighbor searches using uniform grid partitioning
- **Parallel Processing**: Rayon-based parallel force calculations
- **SIMD Operations**: Vectorized Bevy math operations
- **Memory Pooling**: Pre-allocated vectors for performance
- **Rust 2024 Edition**: Leveraging latest language features and performance optimizations

### Boundary Conditions
- **Collision Detection**: Configurable damping for realistic wall interactions
- **Bottom Boundary**: Special handling to prevent particle compression
- **Side Boundaries**: Standard reflection with energy dissipation

## 🎨 Visual Features

- **Particle Rendering**: Sprite-based rendering with size scaling
- **Velocity Coloring**: Blue-to-white gradient based on particle speed
- **Layering**: Z-depth sorting for visual depth
- **Real-time Updates**: Smooth 60+ FPS rendering

## 📊 Performance

- **Particles**: 4,000 simulated particles
- **Target FPS**: 60+ FPS on modern hardware
- **CPU**: Multi-threaded SPH calculations

## 🛠️ Development

### Project Structure
```
src/
├── main.rs          # Application entry point
├── components.rs    # ECS components
├── kernels.rs       # SPH math functions
├── resources/       # Bevy resources
│   ├── mod.rs       # Resource module exports
│   ├── config.rs    # Configuration parameters
│   └── simulation.rs # Simulation state
└── systems.rs       # Bevy systems
```

## 🤝 Contributing

Contributions are welcome! Areas for improvement:
- Surface tension implementation
- Advanced boundary conditions
- Performance optimizations
- Additional SPH kernels

### Development Setup
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `cargo clippy` and `cargo fmt`
5. Submit a pull request

## 📚 References

- [Müller, M., et al. (2003). "Particle-based fluid simulation for interactive applications"](https://matthias-research.github.io/pages/publications/sca03.pdf)

- [Sebastian, Lague. "Coding Adventure: Simulating Fluids"](https://www.youtube.com/watch?v=rSKMYc1CQHE)


## 🙏 Acknowledgments

- **Bevy Game Engine**: High-performance ECS framework
- **Rayon**: Parallel computation library
- **Rust Community**: Excellent documentation and crates


## 📄 License

This project is open source. See LICENSE file for details.

---