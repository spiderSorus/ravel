use bevy::prelude::*;
use crate::resources::SimulationState;
use crate::experiments::ExperimentState;
use ravel_ecs::{MultiverseId, MultiverseState, MultiverseConfig};
use crate::experiments::common::{setup_multiverse_experiment, spawn_ravels};

pub struct MultiverseBPlugin;

impl Plugin for MultiverseBPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ExperimentState::MultiverseB), setup)
           .add_systems(Update, tick.run_if(in_state(ExperimentState::MultiverseB)));
    }
}

fn setup(
    mut commands: Commands,
    mut sim: ResMut<SimulationState>,
    mut multiverse_state: ResMut<MultiverseState>,
    mut active_multiverse: ResMut<ravel_ecs::ActiveMultiverse>,
) {
    spawn_multiverse_b(&mut commands, &mut sim, &mut multiverse_state, &mut active_multiverse);
}

pub fn spawn_multiverse_b(
    commands: &mut Commands,
    sim: &mut SimulationState,
    multiverse_state: &mut MultiverseState,
    active_multiverse: &mut ravel_ecs::ActiveMultiverse,
) {
    let config = MultiverseConfig {
        id: MultiverseId(11),
        name: "Multiverse B".to_string(),
        required_components: vec![],
    };
    
    setup_multiverse_experiment(config, (1, 0.5), sim, multiverse_state, active_multiverse);
    
    let multiverse_id = MultiverseId(11);
    
    // Spawn positive
    spawn_ravels(commands, multiverse_id, 5000, |_| [i32::MAX; 64]);
    // Spawn negative
    spawn_ravels(commands, multiverse_id, 5000, |_| [i32::MIN; 64]);
}

fn tick(
    time: Res<Time>,
    _commands: Commands,
    mut sim: ResMut<SimulationState>,
) {
    sim.timer.tick(time.delta());
    if sim.timer.finished() {
        sim.step_counter += 1;
        // Multiverse B should be in stasis, so no spawning or other state changes here.
        // Keeping timer and step_counter for potential future loading.
    }
}
