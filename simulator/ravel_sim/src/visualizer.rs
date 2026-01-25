use bevy::prelude::*;
use crate::AppState;
use crate::world_view::WorldView;
use ravel_ecs::iching::{get_hexagram_by_gate, RAVE_MANDALA};

use crate::experiments::ExperimentState;

pub struct VisualizerPlugin;

#[derive(Resource, Default)]
pub struct SelectedEntity(pub Option<usize>);

#[derive(Component)]
struct VisualizerUi;

#[derive(Component)]
struct GateCell(i32); // Stores the value to display in tooltip

#[derive(Component)]
struct TooltipUi;

impl Plugin for VisualizerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedEntity>()
           .add_systems(Update, (
               select_entity.run_if(in_state(AppState::Simulation)),
               update_visualizer_ui.run_if(in_state(AppState::Simulation)),
               handle_tooltip.run_if(in_state(AppState::Simulation).and_then(any_with_component::<VisualizerUi>)),
               manage_visualizer_visibility.run_if(in_state(AppState::Simulation)),
           ))
           .add_systems(OnExit(AppState::Simulation), (cleanup_visualizer, reset_selected_entity))
           .add_systems(OnExit(ExperimentState::Genesis), (cleanup_visualizer, reset_selected_entity))
           .add_systems(OnExit(ExperimentState::Entropy), (cleanup_visualizer, reset_selected_entity))
           .add_systems(OnExit(ExperimentState::Equilibrium), (cleanup_visualizer, reset_selected_entity))
           .add_systems(OnExit(ExperimentState::Civilization), (cleanup_visualizer, reset_selected_entity))
           .add_systems(OnExit(ExperimentState::ShowAMap), (cleanup_visualizer, reset_selected_entity))
           .add_systems(OnExit(ExperimentState::PopulationMap), (cleanup_visualizer, reset_selected_entity))
           .add_systems(OnExit(ExperimentState::Chords), (cleanup_visualizer, reset_selected_entity))
           .add_systems(OnExit(ExperimentState::ViewChanger), (cleanup_visualizer, reset_selected_entity))
           .add_systems(OnExit(ExperimentState::MultiverseA), (cleanup_visualizer, reset_selected_entity))
           .add_systems(OnExit(ExperimentState::MultiverseB), (cleanup_visualizer, reset_selected_entity))
           .add_systems(OnExit(ExperimentState::MultiverseSwitcher), (cleanup_visualizer, reset_selected_entity));
    }
}

fn manage_visualizer_visibility(
    world_view: Res<State<WorldView>>,
    mut query: Query<&mut Visibility, With<VisualizerUi>>,
) {
    let should_be_visible = match *world_view.get() {
        WorldView::Standard | WorldView::MandalaOnly => true, // Assuming analysis is part of these
        _ => false,
    };

    let new_vis = if should_be_visible { Visibility::Visible } else { Visibility::Hidden };
    for mut vis in query.iter_mut() {
        if *vis != new_vis {
            *vis = new_vis;
        }
    }
}

fn select_entity(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut selected: ResMut<SelectedEntity>,
    ravels: Query<&ravel_ecs::Ravel, With<ravel_ecs::MultiverseId>>,
    ui_interaction: Query<&Interaction, With<Node>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        // Check if clicking on UI
        for interaction in ui_interaction.iter() {
            if *interaction != Interaction::None {
                return;
            }
        }

        let window = windows.single();
        if let Some(cursor_pos) = window.cursor_position() {
            let (camera, camera_transform) = camera_q.single();
            
            // Map cursor to world
            let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return; };
            
            // Find closest entity
            let mut closest = None;
            let mut min_dist = 20.0; // Click radius
            
            let image_width = 2000.0;
            let image_height = 1478.0;
            let image_aspect = image_width / image_height;
            let window_aspect = window.width() / window.height();

            let (draw_width, draw_height) = if window_aspect > image_aspect {
                (window.height() * image_aspect, window.height())
            } else {
                (window.width(), window.width() / image_aspect)
            };

            for (idx, ravel) in ravels.iter().enumerate() {
                let (x, y) = ravel.pos.to_mercator();
                
                let world_x = (x + std::f32::consts::PI) / (2.0 * std::f32::consts::PI) * draw_width - draw_width / 2.0;
                let world_y = -((y - std::f32::consts::PI) / (2.0 * -std::f32::consts::PI) * draw_height - draw_height / 2.0);
                
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
    query: Query<Entity, With<VisualizerUi>>,
    asset_server: Res<AssetServer>,
    mut last_selected: Local<Option<usize>>,
    ravels: Query<&ravel_ecs::Ravel, With<ravel_ecs::MultiverseId>>,
) {
    // If selection hasn't changed, don't refresh the UI (keeps Tooltips and Interactions stable)
    if *last_selected == selected.0 && !query.is_empty() {
        return;
    }
    *last_selected = selected.0;

    // If nothing selected, remove UI
    let Some(idx) = selected.0 else {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        return;
    };

    // Refresh UI
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Get the nth ravel from the list
    let ravels_vec: Vec<_> = ravels.iter().collect();
    let Some(ravel) = ravels_vec.get(idx) else {
        return;
    };
    
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
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 1.0)),
            ..default()
        },
        VisualizerUi,
        Interaction::default(), // Block clicks
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
            for (_i, &gate) in RAVE_MANDALA.iter().enumerate() {
                let hex = get_hexagram_by_gate(gate);
                let binary_idx = ravel_ecs::iching::gate_to_binary(gate) as usize;
                
                let val = ravel.chord[binary_idx];
                
                let symbol_color = if val >= 0 { // Treat 0 as minimally positive for display
                    // Adjusted intensity calculation for positive values
                    let intensity = (val as f32 / i32::MAX as f32).clamp(0.0, 1.0);
                    Color::srgb(1.0 - intensity, 1.0, 1.0 - intensity) // Greenish
                } else { // val < 0
                    // Adjusted intensity calculation for negative values
                    let intensity = ((val as i128).abs() as f32 / i32::MAX as f32).clamp(0.0, 1.0);
                    Color::srgb(1.0, 1.0 - intensity, 1.0 - intensity) // Reddish
                };

                grid.spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                    Interaction::default(),
                    GateCell(val),
                )).with_children(|cell| {
                    cell.spawn(TextBundle::from_section(
                        hex.character.to_string(),
                        TextStyle { font: symbol_font.clone(), font_size: 40.0, color: symbol_color }
                    ));
                });
            }
        });
    });
}

fn handle_tooltip(
    mut commands: Commands,
    cell_query: Query<(&Interaction, &GateCell)>,
    tooltip_query: Query<Entity, With<TooltipUi>>,
    asset_server: Res<AssetServer>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let mouse_pos = window.cursor_position();

    let mut hovered_val = None;
    for (interaction, cell) in cell_query.iter() {
        if *interaction == Interaction::Hovered {
            hovered_val = Some(cell.0);
            break;
        }
    }

    if let Some(val) = hovered_val {
        if let Some(pos) = mouse_pos {
            // Remove old tooltip
            for entity in tooltip_query.iter() {
                commands.entity(entity).despawn_recursive();
            }

            let font = asset_server.load("FiraSans-Bold.ttf");
            commands.spawn((
                TextBundle::from_section(
                    format!("{}", val),
                    TextStyle { font: font, font_size: 20.0, color: Color::WHITE }
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(pos.x),
                    top: Val::Px(pos.y - 25.0), // Floating just above
                    ..default()
                }),
                TooltipUi,
            ));
        }
    } else {
        // No hover, remove tooltip
        for entity in tooltip_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn cleanup_visualizer(mut commands: Commands, ui_query: Query<Entity, With<VisualizerUi>>, tooltip_query: Query<Entity, With<TooltipUi>>) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in tooltip_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn reset_selected_entity(mut selected: ResMut<SelectedEntity>) {
    selected.0 = None;
}