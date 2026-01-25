use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use ravel_ecs::iching::{gate_to_binary, get_hexagram_by_gate, RAVE_MANDALA};
use crate::AppState;
use crate::world_view::WorldView;
use crate::experiments::ExperimentState;

// To make it reusable, we might define the state generic, but for now we
// just export the systems and let main.rs schedule them.
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
           .add_systems(OnExit(ExperimentState::Genesis), cleanup_mandala)
           .add_systems(OnExit(ExperimentState::Entropy), cleanup_mandala)
           .add_systems(OnExit(ExperimentState::Equilibrium), cleanup_mandala)
           .add_systems(OnExit(ExperimentState::Civilization), cleanup_mandala)
           .add_systems(OnExit(ExperimentState::ShowAMap), cleanup_mandala)
           .add_systems(OnExit(ExperimentState::PopulationMap), cleanup_mandala)
           .add_systems(OnExit(ExperimentState::Chords), cleanup_mandala)
           .add_systems(OnExit(ExperimentState::ViewChanger), cleanup_mandala)
           .add_systems(OnExit(ExperimentState::MultiverseA), cleanup_mandala)
           .add_systems(OnExit(ExperimentState::MultiverseB), cleanup_mandala)
           .add_systems(OnExit(ExperimentState::MultiverseSwitcher), cleanup_mandala)
           .add_systems(Update, (
               update_global_chord.run_if(in_state(AppState::Simulation)),
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

        commands.spawn(( MaterialMesh2dBundle {
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

fn draw_mandala_grid(mut gizmos: Gizmos, world_view: Res<State<WorldView>>, visibility: Res<MandalaVisibility>) {
    let should_draw = match *world_view.get() {
        WorldView::Standard | WorldView::MandalaOnly => visibility.0,
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

fn update_global_chord(
    mut multiverse_state: ResMut<ravel_ecs::MultiverseState>,
    query: Query<(&ravel_ecs::MultiverseId, &ravel_ecs::Ravel)>,
) {
    let multiverse_id = multiverse_state.config.id;
    let mut aggregate = [0i128; 64];
    let mut ravel_count = 0;

    for (id, ravel) in query.iter() {
        if *id == multiverse_id {
            ravel_count += 1;
            for i in 0..64 {
                aggregate[i] += ravel.chord[i] as i128;
            }
        }
    }

    if ravel_count > 0 {
        for i in 0..64 {
            aggregate[i] /= ravel_count as i128;
        }
    }

    multiverse_state.global_chord = aggregate;
}

fn update_sector_colors(
    multiverse_state: Res<ravel_ecs::MultiverseState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&SectorIndex, &Handle<ColorMaterial>)>,
) {
    let chord = &multiverse_state.global_chord;
    let max_possible_avg = i32::MAX as f64; // Max possible average is i32::MAX

    for (sector, mat_handle) in query.iter() {
        let gate = RAVE_MANDALA[sector.0];
        let binary_idx = gate_to_binary(gate) as usize;
        let val = chord[binary_idx] as f64;

        let color = if val.abs() < (max_possible_avg * 0.001) { // Threshold for grey: 0.1% of max_possible_avg
            Color::srgb(0.5, 0.5, 0.5) // Neutral grey
        } else {
            let norm = (val / max_possible_avg).clamp(-1.0, 1.0);
            if norm > 0.0 {
                let t = norm as f32;
                Color::srgb(0.0, 0.5 + t * 0.5, 0.0) // Green intensity
            } else {
                let t = norm.abs() as f32;
                Color::srgb(0.5 + t * 0.5, 0.0, 0.0) // Red intensity
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
    multiverse_state: Res<ravel_ecs::MultiverseState>,
    mut query: Query<(&mut Text, &mut Style, &Visibility), With<PopulationUi>>,
    app_state: Res<State<AppState>>,
    world_view: Res<State<WorldView>>,
    mut timer: Local<Timer>,
    time: Res<Time>,
    ravels: Query<&ravel_ecs::Ravel, With<ravel_ecs::MultiverseId>>,
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
            let total = ravels.iter().count();
            let mut output = format!("Total Population: {}
", total);
            output.push_str("Cumulative Affinity Scores:
");

            let chord = &multiverse_state.global_chord;
            
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
                    let val_str = if val.abs() > 1_000_000_000 { format!("{:.1e}", val as f64) } else { format!("{}", val) };

                    output.push_str(&format!("{} {:>2}: {:<10} ", symbol, gate, val_str));
                }
                output.push('\n'); // Add newline after each row
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