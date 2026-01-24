use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use ravel_ecs::iching::{gate_to_binary, get_hexagram_by_gate, RAVE_MANDALA};
use crate::resources::SimulationState;
use crate::AppState;
use crate::world_view::WorldView;

// To make it reusable, we might define the state generic, but for now we'll import AppState
// or just export the systems and let main.rs schedule them.
// "Make it a view class" -> A Plugin is the closest Bevy equivalent.

pub struct MandalaViewPlugin;

#[derive(Resource, Default)]
struct MandalaVisibility(pub bool);

impl Plugin for MandalaViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MandalaVisibility(true)); // Default visible

        // Startup: UI
        app.add_systems(Startup, setup_population_ui);

        // Simulation Phase
        app.add_systems(OnEnter(AppState::Simulation), setup_mandala)
           .add_systems(OnExit(AppState::Simulation), cleanup_mandala)
           .add_systems(Update, (
               draw_mandala_grid.run_if(in_state(AppState::Simulation).and_then(any_with_component::<MandalaElement>)),
               update_sector_colors.run_if(in_state(AppState::Simulation).and_then(any_with_component::<MandalaElement>)),
               update_population_stats, 
               toggle_population_ui,
               toggle_mandala_view,
               manage_mandala_visibility.run_if(in_state(AppState::Simulation)),
           ));
    }
}

// --- COMPONENTS ---
#[derive(Component)]
pub struct SectorIndex(pub usize);

#[derive(Component)]
pub struct MandalaElement; 

#[derive(Component)]
pub struct PopulationUi;

// --- SYSTEMS ---

pub fn setup_mandala(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    // Sectors
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
            MandalaElement,
        ));
    }

    // Labels
    let label_radius = 330.0;
    let symbol_font = asset_server.load("AppleSymbols.ttf");

    for (i, &gate) in RAVE_MANDALA.iter().enumerate() {
        let angle = ((i as f32 + 0.5) / 64.0) * std::f32::consts::TAU;
        let x = label_radius * angle.cos();
        let y = label_radius * angle.sin();

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    get_hexagram_by_gate(gate).character.to_string(),
                    TextStyle {
                        font: symbol_font.clone(),
                        font_size: 24.0,
                        color: Color::srgb(1.0, 0.0, 0.0),
                    },
                ),
                transform: Transform::from_translation(Vec3::new(x, y, 1.0)),
                ..default()
            },
            MandalaElement,
        ));
    }
}

pub fn cleanup_mandala(
    mut commands: Commands,
    query: Query<Entity, With<MandalaElement>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn draw_mandala_grid(mut gizmos: Gizmos, world_view: Res<State<WorldView>>) {
    let should_draw = match *world_view.get() {
        WorldView::Standard | WorldView::MandalaOnly => true,
        _ => false,
    };

    if !should_draw { return; }

    let center = Vec2::ZERO;
    let radius = 300.0;
    let color = Color::srgb(0.2, 0.0, 0.4); 

    for i in 0..64 {
        let angle = (i as f32 / 64.0) * std::f32::consts::TAU;
        let direction = Vec2::new(angle.cos(), angle.sin());
        gizmos.line_2d(center, center + (direction * radius), color);
    }
    gizmos.circle_2d(center, radius, color);
}

fn update_sector_colors(
    sim: Res<SimulationState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&SectorIndex, &Handle<ColorMaterial>)>,
) {
    let chord = &sim.multiverse.global_chord;
    let max_val = chord.iter().map(|v: &i128| v.abs()).max().unwrap_or(1) as f64;
    let max_val = max_val.max(10.0); 

    for (sector, mat_handle) in query.iter() {
        let gate = RAVE_MANDALA[sector.0];
        let binary_idx = gate_to_binary(gate) as usize;
        let val = chord[binary_idx] as f64; 

        let color = if val.abs() < 0.001 {
            Color::BLACK
        } else {
            let norm = (val / max_val).clamp(-1.0, 1.0);
            if norm > 0.0 {
                let t = norm as f32;
                Color::srgb(0.0, t, 0.0) // Green intensity
            } else {
                let t = norm.abs() as f32;
                Color::srgb(t, 0.0, 0.0) // Red intensity
            }
        };

        if let Some(mat) = materials.get_mut(mat_handle) {
            mat.color = color;
        }
    }
}

// --- POPULATION UI ---

// Setup for Pop UI is still in main.rs (Startup)? Or should be here?
// If this is a "View Class", it should probably manage its own UI creation too.
// Let's move Population UI setup here?
// But `setup_app` in `main.rs` currently does it.
// If we move it here, we can attach it to `Startup` in the Plugin.

pub fn setup_population_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("AppleSymbols.ttf"); // Changed to support hexagrams
    commands.spawn((
        TextBundle::from_section(
            "Population Stats",
            TextStyle {
                font: font.clone(),
                font_size: 14.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(80.0),
            left: Val::Px(10.0),
            display: Display::None, // Hidden by default, toggled by P or State?
            ..default()
        }),
        PopulationUi,
    ));
}

fn update_population_stats(
    sim: Res<SimulationState>,
    mut query: Query<(&mut Text, &mut Style, &Visibility), With<PopulationUi>>,
    app_state: Res<State<AppState>>,
    world_view: Res<State<WorldView>>,
    mut timer: Local<Timer>,
    time: Res<Time>,
) {
    if timer.duration() == std::time::Duration::ZERO {
        *timer = Timer::from_seconds(0.5, TimerMode::Repeating);
    }
    timer.tick(time.delta());

    for (mut text, mut style, _vis) in query.iter_mut() {
        if *app_state.get() == AppState::Menu {
            style.display = Display::None;
            continue;
        }

        let should_show = match *world_view.get() {
            WorldView::Standard | WorldView::MandalaOnly => true,
            _ => false,
        };

        if !should_show {
            style.display = Display::None;
            continue;
        }
        
        if style.display != Display::None && timer.finished() {
            let total = sim.multiverse.world.iter_entities().count();
            let mut output = format!("Total Population: {}\n", total);
            output.push_str("Cumulative Affinity Scores:\n");

            let chord = &sim.multiverse.global_chord;
            
            // 4 columns of 16 rows
            let rows = 16;
            for r in 0..rows {
                for col in 0..4 {
                    let gate_idx = r + (col * 16); // 0-63
                    let gate = (gate_idx + 1) as u8; // Sequential gates 1-64
                    let binary_idx = gate_to_binary(gate) as usize;
                    let val = chord[binary_idx];
                    let symbol = get_hexagram_by_gate(gate).character;

                    // Scientific notation for very large numbers or truncated
                    let val_str = if val.abs() > 1_000_000_000 {
                        format!("{:.1e}", val as f64)
                    } else {
                        format!("{}", val)
                    };

                    output.push_str(&format!("{} {:>2}: {:<10} ", symbol, gate, val_str));
                }
                output.push('\n');
            }

            text.sections[0].value = output;
        }
    }
}

fn toggle_population_ui(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Style, With<PopulationUi>>,
    app_state: Res<State<AppState>>,
) {
    if *app_state.get() == AppState::Menu { return; }

    if keys.just_pressed(KeyCode::KeyP) {
        for mut style in query.iter_mut() {
            if style.display == Display::None {
                style.display = Display::Flex;
            } else {
                style.display = Display::None;
            }
        }
    }
}

fn toggle_mandala_view(
    keys: Res<ButtonInput<KeyCode>>,
    mut visibility_res: ResMut<MandalaVisibility>,
) {
    if keys.just_pressed(KeyCode::KeyT) {
        visibility_res.0 = !visibility_res.0;
    }
}

fn manage_mandala_visibility(
    world_view: Res<State<WorldView>>,
    visibility_res: Res<MandalaVisibility>,
    mut query: Query<&mut Visibility, With<MandalaElement>>,
) {
let should_be_visible = match *world_view.get() {
    WorldView::Standard | WorldView::MandalaOnly => visibility_res.0,
    WorldView::Player => false,
    _ => false,
};

    let new_vis = if should_be_visible { Visibility::Visible } else { Visibility::Hidden };
    for mut vis in query.iter_mut() {
        if *vis != new_vis {
            *vis = new_vis;
        }
    }
}
