use bevy::prelude::*;
use crate::resources::SimulationState;
use crate::experiments::ExperimentState;
use ravel_ecs::{MultiverseId, MultiverseState, MultiverseConfig};
use crate::experiments::common::{setup_multiverse_experiment, spawn_ravels};

pub struct MultiverseAPlugin;

impl Plugin for MultiverseAPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ExperimentState::MultiverseA), setup)
           .add_systems(Update, tick.run_if(in_state(ExperimentState::MultiverseA)));
    }
}

fn setup(
    mut commands: Commands,
    mut sim: ResMut<SimulationState>,
    mut multiverse_state: ResMut<MultiverseState>,
    mut active_multiverse: ResMut<ravel_ecs::ActiveMultiverse>,
) {
    spawn_multiverse_a(&mut commands, &mut sim, &mut multiverse_state, &mut active_multiverse);
}

pub fn spawn_multiverse_a(
    commands: &mut Commands,
    sim: &mut SimulationState,
    multiverse_state: &mut MultiverseState,
    active_multiverse: &mut ravel_ecs::ActiveMultiverse,
) {
    let config = MultiverseConfig {
        id: MultiverseId(10),
        name: "Multiverse A".to_string(),
        required_components: vec![],
    };
    
    setup_multiverse_experiment(config, (1, 1.0), sim, multiverse_state, active_multiverse);
    
    let multiverse_id = MultiverseId(10);
    spawn_ravels(commands, multiverse_id, 10000, |_| [0; 64]);
}

fn tick(
    _time: Res<Time>,
    _commands: Commands,
    _sim: ResMut<SimulationState>,
) {
    // Multiverse A should remain in stasis, so this function does nothing.
}
