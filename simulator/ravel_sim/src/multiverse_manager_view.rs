use bevy::prelude::*;
use crate::AppState;
use crate::experiments::ExperimentState;
use ravel_ecs::MultiverseId;

pub struct MultiverseManagerPlugin;

#[derive(Resource, Default)]
struct MultiverseManagerVisibility(pub bool);

#[derive(Component)]
struct MultiverseManagerUi;

#[derive(Component)]
struct MultiverseButton(u32);

#[derive(Event)]
pub struct LoadMultiverseEvent(pub MultiverseId);

impl Plugin for MultiverseManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MultiverseManagerVisibility(false)) // Hidden by default
           .add_event::<LoadMultiverseEvent>()
           .add_systems(Update, (
               toggle_multiverse_manager.run_if(in_state(ExperimentState::MultiverseSwitcher)),
               manage_manager_ui.run_if(in_state(ExperimentState::MultiverseSwitcher)),
               handle_multiverse_switch.run_if(in_state(ExperimentState::MultiverseSwitcher)),
           ))
           .add_systems(OnExit(AppState::Simulation), cleanup_manager)
           .add_systems(OnExit(ExperimentState::Genesis), cleanup_manager)
           .add_systems(OnExit(ExperimentState::Entropy), cleanup_manager)
           .add_systems(OnExit(ExperimentState::Equilibrium), cleanup_manager)
           .add_systems(OnExit(ExperimentState::Civilization), cleanup_manager)
           .add_systems(OnExit(ExperimentState::ShowAMap), cleanup_manager)
           .add_systems(OnExit(ExperimentState::PopulationMap), cleanup_manager)
           .add_systems(OnExit(ExperimentState::Chords), cleanup_manager)
           .add_systems(OnExit(ExperimentState::ViewChanger), cleanup_manager)
           .add_systems(OnExit(ExperimentState::MultiverseA), cleanup_manager)
           .add_systems(OnExit(ExperimentState::MultiverseB), cleanup_manager)
           .add_systems(OnExit(ExperimentState::MultiverseSwitcher), cleanup_manager);
    }
}

fn toggle_multiverse_manager(
    keys: Res<ButtonInput<KeyCode>>,
    mut visibility: ResMut<MultiverseManagerVisibility>,
) {
    if keys.just_pressed(KeyCode::KeyQ) {
        visibility.0 = !visibility.0;
    }
}

fn manage_manager_ui(
    mut commands: Commands,
    visibility: Res<MultiverseManagerVisibility>,
    mut query: Query<&mut Visibility, With<MultiverseManagerUi>>,
    asset_server: Res<AssetServer>,
) {
    // Get or create the UI panel
    if query.is_empty() && visibility.0 {
        let font = asset_server.load("FiraSans-Bold.ttf");
        
        commands.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(100.0),
                    left: Val::Px(10.0),
                    right: Val::Px(10.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    padding: UiRect::all(Val::Px(15.0)),
                    max_width: Val::Px(400.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
                ..default()
            },
            MultiverseManagerUi,
            Interaction::default(), // Block clicks
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "MULTIVERSE MANAGER (Q to toggle)",
                TextStyle { font: font.clone(), font_size: 16.0, color: Color::WHITE }
            ));
            
            // Multiverse A button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::hsl(200.0, 0.8, 0.6)),
                    ..default()
                },
                MultiverseButton(10),
            )).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Load Multiverse A",
                    TextStyle { font: font.clone(), font_size: 14.0, color: Color::BLACK }
                ));
            });
            
            // Multiverse B button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::hsl(230.0, 0.8, 0.6)),
                    ..default()
                },
                MultiverseButton(11),
            )).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Load Multiverse B",
                    TextStyle { font: font.clone(), font_size: 14.0, color: Color::BLACK }
                ));
            });

            // Multiverse C button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::hsl(260.0, 0.8, 0.6)),
                    ..default()
                },
                MultiverseButton(12),
            )).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Load Multiverse C",
                    TextStyle { font: font.clone(), font_size: 14.0, color: Color::BLACK }
                ));
            });
        });
    } else if !query.is_empty() {
        // Update visibility
        for mut vis in query.iter_mut() {
            *vis = if visibility.0 { Visibility::Visible } else { Visibility::Hidden };
        }
    }
}

fn handle_multiverse_switch(
    mut interaction_query: Query<(&Interaction, &MultiverseButton), Changed<Interaction>>,
    mut load_event: EventWriter<LoadMultiverseEvent>,
) {
    for (interaction, button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match button.0 {
                10 => {
                    // Load Multiverse A
                    load_event.send(LoadMultiverseEvent(MultiverseId(10)));
                }
                11 => {
                    // Load Multiverse B
                    load_event.send(LoadMultiverseEvent(MultiverseId(11)));
                }
                12 => {
                    // Load Multiverse C
                    load_event.send(LoadMultiverseEvent(MultiverseId(12)));
                }
                _ => {}
            }
        }
    }
}

fn cleanup_manager(mut commands: Commands, query: Query<Entity, With<MultiverseManagerUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
