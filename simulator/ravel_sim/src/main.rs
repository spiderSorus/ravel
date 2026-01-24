use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::window::WindowMode;
use bevy::gizmos::config::{DefaultGizmoConfigGroup, GizmoConfigStore};
use ravel_ecs::{Ravel, MultiverseLibrary, ActiveMultiverse};
use std::any::TypeId;

mod resources;
mod experiments;
mod view;
mod map_view;
mod map_resources;
mod visualizer;
mod world_view;
mod multiverse_manager;

use resources::SimulationState;
use experiments::{ExperimentsPlugin, ExperimentState, EXPERIMENTS};
use view::MandalaViewPlugin;
use map_view::MapViewPlugin;
use visualizer::VisualizerPlugin;
use world_view::{WorldViewPlugin, WorldView};

// --- STATES ---
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    Simulation,
}

// --- COMPONENTS ---
#[derive(Component)]
struct MenuUi;

#[derive(Component)]
struct ExperimentButton(usize);

#[derive(Component)]
struct KeymapUi;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ravel: Cosmic Sensor".into(),
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.05)))
        .init_resource::<map_resources::LandMask>()
        .init_resource::<MultiverseLibrary>()
        .init_resource::<ActiveMultiverse>()
        .init_state::<AppState>()
        .add_plugins(WorldViewPlugin)
        .add_plugins(ExperimentsPlugin)
        .add_plugins(MandalaViewPlugin)
        .add_plugins(MapViewPlugin)
        .add_plugins(VisualizerPlugin)
        .add_plugins(bevy_svg::prelude::SvgPlugin)
        
        .add_systems(Startup, (setup_app, setup_keymap_ui, setup_map_resource))
        .add_systems(Update, map_resources::generate_land_mask_system.run_if(resource_exists::<map_resources::MapImageHandle>))
        .add_systems(OnExit(AppState::Simulation), cleanup_simulation)
        .add_systems(Update, (
            quit_simulation,
            return_to_menu,
            toggle_keymap_ui,
            switch_world_view,
        ))
        
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(OnExit(AppState::Menu), cleanup_menu)
        .add_systems(Update, menu_interaction.run_if(in_state(AppState::Menu)))
        
        .run();
}

fn setup_app(
    mut commands: Commands, 
    mut gizmo_config: ResMut<GizmoConfigStore>,
) {
    let (config, _) = gizmo_config.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 1.0;

    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(SimulationState::default());
}

fn setup_map_resource(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("map.png");
    commands.insert_resource(map_resources::MapImageHandle(handle));
}

fn cleanup_simulation(mut active: ResMut<ActiveMultiverse>) {
    active.clear_active();
}

// --- KEYMAP UI ---
fn setup_keymap_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("FiraSans-Bold.ttf");
    let key_style = TextStyle { font: font.clone(), font_size: 16.0, color: Color::WHITE };
    let desc_style = TextStyle { font: font.clone(), font_size: 16.0, color: Color::srgb(0.5, 0.5, 0.5) };

    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                right: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        KeymapUi,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_sections([
            TextSection::new("Z ", key_style.clone()),
            TextSection::new("- Toggle Keymap\n", desc_style.clone()),
            TextSection::new("P ", key_style.clone()),
            TextSection::new("- Toggle Population\n", desc_style.clone()),
            TextSection::new("T ", key_style.clone()),
            TextSection::new("- Toggle Pizza View\n", desc_style.clone()),
            TextSection::new("E ", key_style.clone()),
            TextSection::new("- Toggle Entity View", desc_style.clone()),
        ]));
    });
}

fn toggle_keymap_ui(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Visibility, With<KeymapUi>>) {
    if keys.just_pressed(KeyCode::KeyZ) {
        for mut visibility in query.iter_mut() {
            *visibility = if *visibility == Visibility::Hidden { Visibility::Visible } else { Visibility::Hidden };
        }
    }
}


// --- MENU SYSTEM ---
fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("FiraSans-Bold.ttf");
    let mut sorted_indices: Vec<usize> = (0..EXPERIMENTS.len()).collect();
    sorted_indices.sort_by(|&a, &b| EXPERIMENTS[b].id.cmp(&EXPERIMENTS[a].id));
    
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(10.0),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            ..default()
        },
        MenuUi,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section("SELECT EXPERIMENT", TextStyle { font: font.clone(), font_size: 30.0, color: Color::WHITE }));
        
        for (i, &exp_idx) in sorted_indices.iter().enumerate() {
            let exp = &EXPERIMENTS[exp_idx];
            
            let (bg_color, title_color, desc_color) = if exp.active {
                let hue = if sorted_indices.len() > 1 { i as f32 / (sorted_indices.len() - 1) as f32 * 270.0 } else { 0.0 };
                (Color::hsl(hue, 1.0, 0.5), Color::WHITE, Color::srgba(1.0, 1.0, 1.0, 0.8))
            } else {
                (Color::srgb(0.1, 0.1, 0.1), Color::srgb(0.4, 0.4, 0.4), Color::srgb(0.3, 0.3, 0.3))
            };
            
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(400.0),
                        height: Val::Px(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: BackgroundColor(bg_color),
                    ..default()
                },
                ExperimentButton(exp_idx),
            )).with_children(|parent| {
                parent.spawn(TextBundle::from_section(format!("{}. {}", exp.id, exp.name), TextStyle { font: font.clone(), font_size: 28.0, color: title_color }));
                parent.spawn(TextBundle::from_section(exp.description, TextStyle { font: font.clone(), font_size: 16.0, color: desc_color }));
            });
        }
    });
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn menu_interaction(
    mut interaction_query: Query<(&Interaction, &ExperimentButton), (Changed<Interaction>, With<Button>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_exp_state: ResMut<NextState<ExperimentState>>,
) {
    for (interaction, button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if EXPERIMENTS[button.0].active {
                next_app_state.set(AppState::Simulation);
                next_exp_state.set(EXPERIMENTS[button.0].state);
            }
        }
    }
}

fn return_to_menu(
    keys: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_exp_state: ResMut<NextState<ExperimentState>>,
    mut next_world_view: ResMut<NextState<WorldView>>,
) {
    if keys.just_pressed(KeyCode::Escape) && *app_state.get() == AppState::Simulation {
        next_app_state.set(AppState::Menu);
        next_exp_state.set(ExperimentState::None);
        next_world_view.set(WorldView::None);
    } 
}

fn switch_world_view(
    keys: Res<ButtonInput<KeyCode>>,
    _current_view: Res<State<WorldView>>,
    mut next_view: ResMut<NextState<WorldView>>,
    app_state: Res<State<AppState>>,
) {
    if *app_state.get() != AppState::Simulation { return; }

    if keys.just_pressed(KeyCode::Digit1) {
        next_view.set(WorldView::Standard);
    } else if keys.just_pressed(KeyCode::Digit2) {
        next_view.set(WorldView::MapOnly);
    } else if keys.just_pressed(KeyCode::Digit3) {
        next_view.set(WorldView::MandalaOnly);
    }
}

fn quit_simulation(
    keys: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::Escape) && *app_state.get() == AppState::Menu {
        exit.send(AppExit::Success);
    }
}
