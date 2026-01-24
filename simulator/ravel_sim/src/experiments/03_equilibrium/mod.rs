use bevy::prelude::*;
use crate::resources::SimulationState;
use crate::experiments::ExperimentState;
use ravel_ecs::{Spawner, RavelType, MultiverseId, MultiverseState};

pub struct EquilibriumPlugin;

impl Plugin for EquilibriumPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ExperimentState::Equilibrium), setup)
           .add_systems(Update, tick.run_if(in_state(ExperimentState::Equilibrium)));
    }
}

fn setup(
    mut commands: Commands,
    mut sim: ResMut<SimulationState>,
    mut multiverse_state: ResMut<MultiverseState>,
) {
    multiverse_state.reset_chord();
    sim.current_resolution = 8;
    sim.timer = Timer::from_seconds(2.0, TimerMode::Repeating);

    let multiverse_id = MultiverseId(3); // Equilibrium multiverse ID
    
    for _ in 0..200 {
        let plant = Spawner::spawn(RavelType::Plant, None);
        commands.spawn((plant, multiverse_id));
    }
}

fn tick(
    time: Res<Time>,
    mut commands: Commands,
    mut sim: ResMut<SimulationState>,
) {
    sim.timer.tick(time.delta());
    if sim.timer.finished() {
        let multiverse_id = MultiverseId(3);
        let _target_pop = 200;
        
        // For now, just spawn to maintain population
        for _ in 0..10 {
            let plant = Spawner::spawn(RavelType::Plant, None);
            commands.spawn((plant, multiverse_id));
        }
    }
}
