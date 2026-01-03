mod components;
mod kernels;
mod resources;
mod systems;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowResolution},
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::ResourceInspectorPlugin};

use resources::{FluidConfig, FluidSimulation, BOUNDARY_HEIGHT, BOUNDARY_WIDTH};
use systems::*;

/// Main entry point for the fluid simulation application.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2D Fluid Simulation".into(),
                resolution: WindowResolution::new(
                    BOUNDARY_WIDTH as u32 + 50,
                    BOUNDARY_HEIGHT as u32 + 50,
                ),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .add_plugins(ResourceInspectorPlugin::<FluidConfig>::default())
        .add_plugins((
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .init_resource::<FluidConfig>()
        .insert_resource(FluidSimulation::new())
        .add_systems(Startup, setup_scene)
        .add_systems(
            Update,
            (handle_input, update_physics_rayon, sync_rendering).chain(),
        )
        .run();
}
