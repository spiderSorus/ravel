use bevy::prelude::*;
use crate::resources::SimulationState;
use crate::experiments::ExperimentState;
use ravel_ecs::{Spawner, RavelType, MultiverseId, MultiverseState};

pub struct EntropyPlugin;

impl Plugin for EntropyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ExperimentState::Entropy), setup)
           .add_systems(Update, tick.run_if(in_state(ExperimentState::Entropy)));
    }
}

fn setup(
    mut sim: ResMut<SimulationState>,
    mut multiverse_state: ResMut<MultiverseState>,
) {
    multiverse_state.reset_chord();
    sim.current_resolution = 32;
    sim.timer = Timer::from_seconds(0.1, TimerMode::Repeating);
}

fn tick(
    time: Res<Time>,
    mut commands: Commands,
    mut sim: ResMut<SimulationState>,
) {
    sim.timer.tick(time.delta());
    if sim.timer.finished() {
        let multiverse_id = MultiverseId(2);
        
        // Spawn high res noise
        for _ in 0..100 {
            let plant = Spawner::spawn(RavelType::Plant, None);
            commands.spawn((plant, multiverse_id));
        }
    }
}
