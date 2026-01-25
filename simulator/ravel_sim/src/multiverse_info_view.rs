use bevy::prelude::*;
use crate::AppState;
use crate::experiments::ExperimentState;
use ravel_ecs::MultiverseState;

pub struct MultiverseInfoViewPlugin;

#[derive(Component)]
struct MultiverseInfoPanel;

#[derive(Component)]
struct MultiverseInfoText;

impl Plugin for MultiverseInfoViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Simulation), setup_multiverse_info)
            .add_systems(Update, update_multiverse_info.run_if(in_state(AppState::Simulation)))
            .add_systems(OnExit(AppState::Simulation), cleanup_multiverse_info)
            .add_systems(OnExit(ExperimentState::Genesis), cleanup_multiverse_info)
            .add_systems(OnExit(ExperimentState::Entropy), cleanup_multiverse_info)
            .add_systems(OnExit(ExperimentState::Equilibrium), cleanup_multiverse_info)
            .add_systems(OnExit(ExperimentState::Civilization), cleanup_multiverse_info)
            .add_systems(OnExit(ExperimentState::ShowAMap), cleanup_multiverse_info)
            .add_systems(OnExit(ExperimentState::PopulationMap), cleanup_multiverse_info)
            .add_systems(OnExit(ExperimentState::Chords), cleanup_multiverse_info)
            .add_systems(OnExit(ExperimentState::ViewChanger), cleanup_multiverse_info)
            .add_systems(OnExit(ExperimentState::MultiverseA), cleanup_multiverse_info)
            .add_systems(OnExit(ExperimentState::MultiverseB), cleanup_multiverse_info)
            .add_systems(OnExit(ExperimentState::MultiverseSwitcher), cleanup_multiverse_info);
    }
}

fn setup_multiverse_info(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("FiraSans-Bold.ttf");
    
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                padding: UiRect::all(Val::Px(12.0)),
                row_gap: Val::Px(4.0),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            ..default()
        },
        MultiverseInfoPanel,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "MULTIVERSE",
            TextStyle { font: font.clone(), font_size: 12.0, color: Color::srgb(0.6, 0.6, 0.6) }
        ));
        
        parent.spawn((
            TextBundle::from_section(
                "None",
                TextStyle { font: font.clone(), font_size: 16.0, color: Color::srgb(0.2, 0.8, 0.6) }
            ),
            MultiverseInfoText,
        ));
    });
}

fn update_multiverse_info(
    multiverse_state: Res<MultiverseState>,
    mut query: Query<&mut Text, With<MultiverseInfoText>>,
) {
    if multiverse_state.is_changed() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!(
                "{} (ID: {})",
                multiverse_state.config.name, multiverse_state.config.id.0
            );
        }
    }
}

fn cleanup_multiverse_info(mut commands: Commands, query: Query<Entity, With<MultiverseInfoPanel>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
