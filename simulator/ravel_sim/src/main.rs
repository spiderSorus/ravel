use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::window::WindowMode;
use bevy::gizmos::config::{DefaultGizmoConfigGroup, GizmoConfigStore};
use bevy::render::render_resource::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use ravel_ecs::{Multiverse, Spawner};
use ravel_ecs::iching::{gate_to_binary, RAVE_MANDALA};

// --- COMPONENTS ---
#[derive(Component)]
struct SectorIndex(usize);

// --- RESOURCES ---
#[derive(Resource)]
struct SimulationState {
    universe: Multiverse,
    timer: Timer,
    current_resolution: u8,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ravel: Cosmic Sensor".into(),
                mode: WindowMode::BorderlessFullscreen, // Maximized
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.05))) // Deep Space Background
        .add_systems(Startup, setup)
        .add_systems(Update, (simulation_tick, draw_mandala, update_sector_colors, quit_simulation))
        .run();
}

fn setup(
    mut commands: Commands, 
    mut gizmo_config: ResMut<GizmoConfigStore>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    // Configure Line Drawing (The Pizza)
    let (config, _) = gizmo_config.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 3.0;

    // 1. Spawn Camera (2D)
    commands.spawn(Camera2dBundle::default());

    // 2. Spawn Sectors (Background)
    let radius = 300.0;
    for i in 0..64 {
        let angle_start = (i as f32 / 64.0) * std::f32::consts::TAU;
        let angle_end = ((i + 1) as f32 / 64.0) * std::f32::consts::TAU;
        
        let v0 = Vec3::ZERO;
        let v1 = Vec3::new(angle_start.cos() * radius, angle_start.sin() * radius, 0.0);
        let v2 = Vec3::new(angle_end.cos() * radius, angle_end.sin() * radius, 0.0);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![
            [v0.x, v0.y, v0.z],
            [v1.x, v1.y, v1.z],
            [v2.x, v2.y, v2.z],
        ]);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(mesh)),
                material: materials.add(ColorMaterial::from(Color::BLACK)), 
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
                ..default()
            },
            SectorIndex(i),
        ));
    }

    // 3. Spawn Gate Labels
    let label_radius = 330.0;
    let font = asset_server.load("FiraSans-Bold.ttf");

    for (i, &gate) in RAVE_MANDALA.iter().enumerate() {
        let angle = ((i as f32 + 0.5) / 64.0) * std::f32::consts::TAU;
        let x = label_radius * angle.cos();
        let y = label_radius * angle.sin();

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("{}", gate),
                TextStyle {
                    font: font.clone(),
                    font_size: 14.0,
                    color: Color::rgb(0.7, 0.7, 0.7),
                },
            ),
            transform: Transform::from_translation(Vec3::new(x, y, 1.0)),
            ..default()
        });
    }

    // 4. Initialize the Physics Engine
    let mut universe = Multiverse::new();
    
    // Pre-seed it slightly
    for _ in 0..100 {
        universe.spawn(Spawner::spawn_plant(1)); // Start at Res 1
    }

    commands.insert_resource(SimulationState {
        universe,
        timer: Timer::from_seconds(2.0, TimerMode::Repeating), // Evolve every 2s
        current_resolution: 1,
    });
}

// --- SYSTEM: THE PHYSICS LOOP ---
fn simulation_tick(
    time: Res<Time>,
    mut sim: ResMut<SimulationState>,
) {
    // 1. Tick the Evolution Timer
    sim.timer.tick(time.delta());
    if sim.timer.finished() {
        // Bump Resolution: 1 -> 2 -> 4 -> 8...
        if sim.current_resolution < 64 {
            sim.current_resolution *= 2;
        } else {
            sim.current_resolution = 1; // Loop back for demo
            sim.universe.entities.clear();
        }
        
        // Spawn new generation
        for _ in 0..50 {
             let plant = Spawner::spawn_plant(sim.current_resolution);
             sim.universe.spawn(plant);
        }
    }

    // 2. ALWAYS SAMPLE (Every Frame)
    // The sensor reads the graph constantly
    sim.universe.sample_graph();
}

// --- SYSTEM: THE VISUALIZATION ---
fn draw_mandala(
    mut gizmos: Gizmos, 
    sim: Res<SimulationState>
) {
    let chord = &sim.universe.global_chord;
    let center = Vec2::ZERO;
    let radius = 300.0;
    let grid_color = Color::rgb(0.3, 0.3, 0.3); // Light Grey Outline

    // 1. Draw Static Sector Outlines (The Grid)
    for i in 0..64 {
        let angle = (i as f32 / 64.0) * std::f32::consts::TAU;
        let direction = Vec2::new(angle.cos(), angle.sin());
        
        gizmos.line_2d(
            center, 
            center + (direction * radius),
            grid_color
        );
    }
    
    // Draw Rings
    gizmos.circle_2d(center, 50.0, grid_color);
    gizmos.circle_2d(center, radius, grid_color);

    // 2. Draw Dynamic Data
    for (i, &gate) in RAVE_MANDALA.iter().enumerate() {
        let angle = ((i as f32 + 0.5) / 64.0) * std::f32::consts::TAU;
        let direction = Vec2::new(angle.cos(), angle.sin());

        let binary_idx = gate_to_binary(gate) as usize;
        let raw_val = chord[binary_idx];
        let strength = raw_val.abs() as f32;

        let len = 50.0 + (strength * 2.0).min(radius - 50.0); 

        let color = if raw_val > 0 {
            Color::GOLD
        } else {
            Color::TURQUOISE
        };

        gizmos.line_2d(
            center + (direction * 50.0),
            center + (direction * len),
            color
        );

        if gate == 1 || gate == 2 {
             gizmos.circle_2d(center + (direction * (len + 20.0)), 10.0, Color::WHITE);
        }
    }
}

fn update_sector_colors(
    sim: Res<SimulationState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&SectorIndex, &Handle<ColorMaterial>)>,
) {
    let chord = &sim.universe.global_chord;
    let max_val = chord.iter().map(|v| v.abs()).max().unwrap_or(1) as f32;
    let max_val = max_val.max(100.0); 

    for (sector, mat_handle) in query.iter() {
        let gate = RAVE_MANDALA[sector.0];
        let binary_idx = gate_to_binary(gate) as usize;
        let val = chord[binary_idx].abs() as f32;

        let intensity = (val / max_val).clamp(0.0, 1.0);
        let color = Color::rgb(intensity, intensity, intensity);

        if let Some(mat) = materials.get_mut(mat_handle) {
            mat.color = color;
        }
    }
}

fn quit_simulation(
    keys: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
