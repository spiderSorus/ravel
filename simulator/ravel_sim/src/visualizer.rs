use bevy::prelude::*;
use crate::resources::SimulationState;
use crate::AppState;
use ravel_ecs::iching::{get_hexagram_by_gate, RAVE_MANDALA};

pub struct VisualizerPlugin;

#[derive(Resource, Default)]
pub struct SelectedEntity(pub Option<usize>);

#[derive(Component)]
struct VisualizerUi;

impl Plugin for VisualizerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedEntity>()
           .add_systems(Update, (
               select_entity.run_if(in_state(AppState::Simulation)),
               update_visualizer_ui.run_if(in_state(AppState::Simulation)),
           ))
           .add_systems(OnExit(AppState::Simulation), cleanup_visualizer);
    }
}

fn select_entity(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut selected: ResMut<SelectedEntity>,
    sim: Res<SimulationState>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let window = windows.single();
        if let Some(cursor_pos) = window.cursor_position() {
            let (camera, camera_transform) = camera_q.single();
            
            // Map cursor to world
            let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return; };
            
            // Find closest entity
            let mut closest = None;
            let mut min_dist = 20.0; // Click radius
            
            let width = window.width();
            let height = window.height();

            for (idx, entity) in sim.universe.entities.iter().enumerate() {
                // Approximate mapping from lon/lat to world coords (simplified)
                // lon -180..180 -> screen width, lat -90..90 -> screen height
                // We use the same projection as map_view for consistency
                let (x, y) = entity.pos.to_mercator();
                
                let world_x = (x + std::f32::consts::PI) / (2.0 * std::f32::consts::PI) * width - width / 2.0;
                let world_y = (y - std::f32::consts::PI) / (2.0 * -std::f32::consts::PI) * height - height / 2.0;
                
                let dist = world_pos.distance(Vec2::new(world_x, world_y));
                if dist < min_dist {
                    min_dist = dist;
                    closest = Some(idx);
                }
            }
            
            selected.0 = closest;
        }
    }
}

fn update_visualizer_ui(
    mut commands: Commands,
    selected: Res<SelectedEntity>,
    sim: Res<SimulationState>,
    query: Query<Entity, With<VisualizerUi>>,
    asset_server: Res<AssetServer>,
) {
    // If nothing selected, remove UI
    let Some(idx) = selected.0 else {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        return;
    };

    // If selected index is invalid (e.g. simulation reset), remove UI
    if idx >= sim.universe.entities.len() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        return;
    }

    // Refresh UI (despawn and respawn for simplicity in this version)
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let entity_data = &sim.universe.entities[idx];
    let font = asset_server.load("FiraSans-Bold.ttf");
    let symbol_font = asset_server.load("AppleSymbols.ttf");

    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                right: Val::Px(0.0),
                width: Val::Percent(50.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
            ..default()
        },
        VisualizerUi,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            format!("ENTITY #{} - CHORD VISUALIZER", idx),
            TextStyle { font: font.clone(), font_size: 24.0, color: Color::WHITE }
        ));

        // Grid of 64 I Ching cards
        parent.spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::px(8, 60.0),
                row_gap: Val::Px(10.0),
                column_gap: Val::Px(10.0),
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            },
            ..default()
        }).with_children(|grid| {
            // We use RAVE_MANDALA order
            for (i, &gate) in RAVE_MANDALA.iter().enumerate() {
                let hex = get_hexagram_by_gate(gate);
                let available = (i as u8) < entity_data.resolution;
                
                let val_bool = if available { entity_data.chord[i] > 0 } else { false };
                
                let symbol_color = if available {
                    if val_bool {
                        Color::srgb(0.0, 1.0, 0.0) // Green
                    } else {
                        Color::srgb(1.0, 0.0, 0.0) // Red
                    }
                } else {
                    Color::srgb(0.3, 0.3, 0.3) // Grey
                };

                grid.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                }).with_children(|cell| {
                    cell.spawn(TextBundle::from_section(
                        hex.character.to_string(),
                        TextStyle { font: symbol_font.clone(), font_size: 40.0, color: symbol_color }
                    ));
                });
            }
        });
    });
}

fn cleanup_visualizer(mut commands: Commands, query: Query<Entity, With<VisualizerUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
