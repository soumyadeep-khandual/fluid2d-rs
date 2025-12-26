use core::f32;

use bevy::{
    // math::{Isometry2d, VectorSpace},
    // dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
    prelude::*,
    window::WindowResolution,
};

use rand::Rng;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

#[derive(Resource, Clone)]
struct SimConfig {
    gravity: Vec3,
    dt: f32,
    restitution: Vec3,
    width: u32,
    height: u32,
    radius: f32,
    influence_radius: f32,
    density_target: f32,
    mass: f32,
    pressure_multiplier: f32,
    particle_count: usize,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0.0, 0.0, 0.0),
            dt: 0.16,
            restitution: Vec3::new(0.7, 0.7, 1.0),
            width: WIDTH,
            height: HEIGHT,
            radius: 2.0,
            influence_radius: 50.0,
            mass: 1.0,
            pressure_multiplier: 1.0,
            density_target: 1.0,
            particle_count: 2_000,
        }
    }
}

#[derive(Component)]
pub struct EntityCountText;

#[derive(Component)]
pub struct SideBar;

#[derive(Component)]
pub struct Particle;

#[derive(Component, Default)]
pub struct Velocity(Vec3);

#[derive(Component, Default)]
pub struct Density(f32);

#[derive(Component, Default)]
pub struct Pressure(f32);

#[derive(Component, Default)]
pub struct PressureGradient(Vec3);

#[derive(Component)]
pub struct DesiredDensityText;

fn spawn_layout(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: ResMut<SimConfig>,
) {
    commands.spawn(Camera2d);
    commands
        .spawn((
            Node {
                display: Display::Grid,
                // Make node fill the entirety of its parent (in this case the window)
                width: percent(100),
                height: percent(100),
                grid_auto_rows: GridTrack::min_content(),
                padding: UiRect::all(Val::Percent(1.0)),
                ..default()
            },
            SideBar,
        ))
        .with_children(|builder| {
            sidebar_text(builder, format!("Entity Count: {:05}", 0), EntityCountText);
            sidebar_text(
                builder,
                format!("Desired Density: {:.5}", 0.0),
                DesiredDensityText,
            );
        });

    let particle_mesh = meshes.add(Circle::new(config.radius));

    // prepare a non-deterministic random number generator:
    let mut rng = rand::rng();
    let max_width = config.width as f32 / 2.0 - config.radius;
    let max_height = config.height as f32 / 2.0 - config.radius;
    let spacing: f32 = 5.0;

    for idx in 0..config.particle_count {
        let (row, col) = (idx / 40, idx % 40);

        commands.spawn((
            Mesh2d(particle_mesh.clone()),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::WHITE))),
            Transform::from_translation(Vec3 {
                x: 100.0 - row as f32 * spacing,
                y: 100.0 - col as f32 * spacing,
                z: 0.0,
            }),
            // Transform::from_translation(Vec3 {
            //     x: rng.random_range(-max_width..=max_width),
            //     y: rng.random_range(-max_height..=max_height),
            //     z: 0.0,
            // }),
            // Velocity(Vec3 {
            //     x: rng.random_range(-1.0..1.0),
            //     y: rng.random_range(-1.0..1.0),
            //     z: 0.0,
            // }),
            Velocity::default(),
            Density::default(),
            Pressure::default(),
            PressureGradient::default(),
            Particle,
        ));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2D Fluid Simulation".to_string(),
                resolution: WindowResolution::new(WIDTH, HEIGHT),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(SimConfig::default())
        .add_systems(Startup, (spawn_layout,))
        .add_systems(
            Update,
            (
                update_entity_count_ui,
                update_target_density_ui,
                update_particle,
                update_density,
                update_pressure_gradient,
                check_reset_request,
                print_dbg,
                // update_color,
                
            ),
        )
        .run();
}

fn update_target_density_ui(
    mut query: Query<&mut Text, With<DesiredDensityText>>,
    keycode: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<SimConfig>,
) {
    if keycode.just_pressed(KeyCode::KeyD) {
        if keycode.pressed(KeyCode::ShiftLeft) {
            config.density_target -= 1.0;
        } else if keycode.pressed(KeyCode::Space) {
            config.density_target = 0.0;
        } else {
            config.density_target += 1.0;
        }
    }
    if let Ok(mut text) = query.single_mut() {
        text.0 = format!("Target Density: {}", config.density_target as i32);
    }
}

fn update_entity_count_ui(
    entities: Query<Entity>,
    mut query: Query<&mut Text, With<EntityCountText>>,
) {
    if let Ok(mut text) = query.single_mut() {
        text.0 = format!("Entity Count: {}", entities.iter().len());
    }
}
fn sidebar_text(builder: &mut ChildSpawnerCommands, text: String, component_tag: impl Component) {
    builder.spawn((
        Text::new(text),
        TextColor::WHITE,
        TextFont::from_font_size(12.0),
        component_tag,
    ));
}

fn update_particle(
    mut query: Query<(&mut Transform, &mut Velocity), With<Particle>>,
    config: Res<SimConfig>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        velocity.0 += config.gravity * config.dt;
        transform.translation += velocity.0 * config.dt;

        let x_component = transform.translation.dot(Vec3::X);
        let y_component = transform.translation.dot(Vec3::Y);
        let max_width = WIDTH as f32 / 2.0 - config.radius;
        let max_height = HEIGHT as f32 / 2.0 - config.radius;

        if x_component.abs() > max_width {
            transform.translation.x = x_component.signum() * max_width;
            velocity.0.x *= -config.restitution.x;
        }
        if y_component.abs() > max_height {
            transform.translation.y = y_component.signum() * max_height;
            velocity.0.y *= -config.restitution.y;
        }
        if velocity.0.length() < 1e-3 {
            velocity.0 = Vec3::ZERO;
        }
        if transform.translation.length() < 1e-3 {
            transform.translation = Vec3::ZERO;
        }
    }
}

fn update_density(
    mut query: Query<(&mut Density, &Transform, &Velocity), With<Particle>>,
    config: Res<SimConfig>,
) {
    let positions: Vec<Vec3> = query
        .iter()
        .map(|t| t.1.translation + t.2.0 * config.dt)
        .collect();

    query.par_iter_mut().for_each(|mut t| {
        let position1 = t.1.translation + t.2.0 * config.dt;
        t.0.0 = 0.0;
        for position2 in positions.iter() {
            let distance = (position1 - position2).length();
            t.0.0 += smoothing_kernel(distance, config.influence_radius) * config.mass;
        }
    });
}

fn update_pressure_gradient(
    mut query: Query<
        (
            &mut Pressure,
            &mut PressureGradient,
            &Density,
            &Transform,
            &mut Velocity,
        ),
        With<Particle>,
    >,
    config: Res<SimConfig>,
) {
    let density_position_array: Vec<(f32, Vec3)> = query
        .iter()
        .map(|t| (t.2.0, t.3.translation + t.4.0 * config.dt))
        .collect();

    query.par_iter_mut().for_each(|mut t| {
        let position1 = t.3.translation + t.4.0 * config.dt;
        let density1 = t.2.0;
        let mut rng = rand::rng();
        t.0.0 = pressure_from_density(density1, config.density_target, config.pressure_multiplier);
        t.1.0 = Vec3::ZERO;

        for (density2, position2) in density_position_array.iter() {
            let diff = position2 - position1;
            let distance = diff.length();
            let dir = diff.normalize_or(Vec3::new(
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
                0.0,
            ));
            let slope = smoothing_kernel_gradient(distance, config.influence_radius);
            let pressure_influence =
                pressure_from_density(*density2, config.density_target, config.pressure_multiplier);
            t.1.0 += -0.5 * (pressure_influence + t.0.0) * dir * slope * config.mass / density2;
        }
        // update velocity
        // let (dir, magnitude) = ((queryitem.1.0 / density1) * config.dt).normalize_and_length();
        // queryitem.4.0 += (stable_sigmoid(magnitude)* 2.0 - 1.0) * dir;
        t.4.0 += (t.1.0 / density1) * config.dt;
    });
}

fn check_reset_request(
    config: ResMut<SimConfig>,
    keycode: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &mut Velocity,
            &mut Transform,
            &mut Density,
            &mut Pressure,
            &mut PressureGradient,
        ),
        With<Particle>,
    >,
) {
    if keycode.just_pressed(KeyCode::KeyR) {
        for (idx, mut i) in query.iter_mut().enumerate() {
            let (row, col) = (idx / 40, idx % 40);
            let spacing: f32 = 5.0;

            i.0.0 = Vec3::ZERO;
            i.1.translation = Vec3 {
                x: 100.0 - row as f32 * spacing,
                y: 100.0 - col as f32 * spacing,
                z: 0.0,
            };
            i.2.0 = 0.0;
            i.3.0 = 0.0;
            i.4.0 = Vec3::ZERO;
        }
    }
}

fn print_dbg(
    keycode: Res<ButtonInput<KeyCode>>,
    query: Query<
        (
            &Velocity,
            &Transform,
            &Density,
            &Pressure,
            &PressureGradient,
        ),
        With<Particle>,
    >,
) {
    if keycode.just_pressed(KeyCode::Backquote) {
        for i in query.iter() {
            println!(
                "v = {:.4}, pos = {:.4}, d = {:.4}, p = {:.4}, pGrad = {:.4}",
                i.0.0.xy(),
                i.1.translation.xy(),
                i.2.0,
                i.3.0,
                i.4.0.xy()
            )
        }
    }
}

fn update_color(
    query: Query<(&MeshMaterial2d<ColorMaterial>, &Velocity), With<Particle>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let v_max_length = query.iter().map(|(_, v)| v.0.length()).reduce(f32::max).expect("inf velocity");
    let blue = Color::srgb(0.0, 0.0, 1.0);
    let red = Color::srgb(1.0, 0.0, 0.0);
    for (material_handle, velocity) in query.iter() {
        if let Some(material) = materials.get_mut(material_handle.0.id()) {
            material.color = blue.mix(&red, (velocity.0.length() / v_max_length).clamp(0.01,0.99));
        }
    }
}

fn smoothing_kernel(distance: f32, influence_radius: f32) -> f32 {
    if distance > influence_radius {
        return 0.0 as f32;
    }
    let norm = f32::consts::PI * influence_radius.powi(4) / 2.0;
    (influence_radius - distance).powi(3) / norm
}

fn smoothing_kernel_gradient(distance: f32, influence_radius: f32) -> f32 {
    if distance > influence_radius {
        return 0.0 as f32;
    }
    let norm = f32::consts::PI * influence_radius.powi(4) / 2.0;
    -3.0 * (influence_radius - distance).powi(3) / norm
}

fn pressure_from_density(density: f32, density_target: f32, multipier: f32) -> f32 {
    multipier * (density_target - density)
}

fn stable_sigmoid(x: f32) -> f32 {
    if x >= 0.0 {
        1.0 / (1.0 + (-x).exp())
    } else {
        let z = x.exp();
        z / (1.0 + z)
    }
}

fn velocity_to_color(v: f32) -> Color {
    let t = stable_sigmoid(v) * 2.0;

    let blue = Color::srgb(0.0, 0.0, 1.0);
    let red = Color::srgb(1.0, 0.0, 0.0);

    blue.mix(&red, t)
}
