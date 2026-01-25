use bevy::prelude::*;
use crate::experiments::ExperimentState;
use ravel_ecs::{MultiverseId, MultiverseState, ActiveMultiverse};
use crate::multiverse_manager_view::LoadMultiverseEvent;
use crate::experiments::multiverse_a::spawn_multiverse_a;
use crate::experiments::multiverse_b::spawn_multiverse_b;
use crate::experiments::multiverse_c::spawn_multiverse_c;
use crate::resources::SimulationState;
use crate::map_resources::LandMask;

pub struct MultiverseSwitcherPlugin;

impl Plugin for MultiverseSwitcherPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ExperimentState::MultiverseSwitcher), setup)
           .add_systems(OnExit(ExperimentState::MultiverseSwitcher), cleanup)
           .add_systems(Update, handle_load_requests.run_if(in_state(ExperimentState::MultiverseSwitcher)));
    }
}

fn setup(
    mut commands: Commands,
    mut sim: ResMut<SimulationState>,
    mut multiverse_state: ResMut<MultiverseState>,
    mut active_multiverse: ResMut<ActiveMultiverse>,
    land_mask: Res<LandMask>,
) {
    // Start with Multiverse C
    spawn_multiverse_c(&mut commands, &mut sim, &mut multiverse_state, &mut active_multiverse, &land_mask);
}

fn handle_load_requests(
    mut events: EventReader<LoadMultiverseEvent>,
    mut commands: Commands,
    mut sim: ResMut<SimulationState>,
    mut multiverse_state: ResMut<MultiverseState>,
    mut active_multiverse: ResMut<ActiveMultiverse>,
    query: Query<(Entity, &MultiverseId)>,
    land_mask: Res<LandMask>,
) {
    for event in events.read() {
        // 1. Cleanup existing
        if let Some(current_id) = active_multiverse.id {
             crate::multiverse_manager::unload_multiverse(current_id, &mut commands, &query);
        }
        
        // 2. Load new
        match event.0.0 {
            10 => spawn_multiverse_a(&mut commands, &mut sim, &mut multiverse_state, &mut active_multiverse),
            11 => spawn_multiverse_b(&mut commands, &mut sim, &mut multiverse_state, &mut active_multiverse),
            12 => spawn_multiverse_c(&mut commands, &mut sim, &mut multiverse_state, &mut active_multiverse, &land_mask),
            _ => warn!("Unknown multiverse ID requested: {}", event.0.0),
        }
    }
}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<MultiverseId>>,
) {
    // Clean up all multiverse entities
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
