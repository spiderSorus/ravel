use bevy::prelude::*;
use crate::resources::SimulationState;
use crate::experiments::ExperimentState;
use ravel_ecs::{Spawner, RavelType, MultiverseId, MultiverseState};

pub struct CivilizationPlugin;

impl Plugin for CivilizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ExperimentState::Civilization), setup)
           .add_systems(Update, tick.run_if(in_state(ExperimentState::Civilization)));
    }
}

fn setup(
    mut commands: Commands,
    mut sim: ResMut<SimulationState>,
    mut multiverse_state: ResMut<MultiverseState>,
) {
    multiverse_state.reset_chord();
    sim.current_resolution = 64;
    sim.timer = Timer::from_seconds(0.01, TimerMode::Repeating);
    sim.step_counter = 0;

    let multiverse_id = MultiverseId(4); // Civilization multiverse ID
    
    for _ in 0..20 {
        let plant = Spawner::spawn(RavelType::Plant, None);
        commands.spawn((plant, multiverse_id));
    }

    for _ in 0..980 {
        let plant = Spawner::spawn(RavelType::Plant, None);
        commands.spawn((plant, multiverse_id));
    }
}

fn tick(
    time: Res<Time>,
    mut commands: Commands,
    mut sim: ResMut<SimulationState>,
    multiverse_state: Res<MultiverseState>,
) {
    sim.timer.tick(time.delta());
    
    if sim.timer.finished() {
        let multiverse_id = MultiverseId(4);
        let batch_size = 100;
        
        for _ in 0..batch_size {
            let entity = Spawner::spawn(RavelType::Balancer, Some(&multiverse_state.global_chord));
            commands.spawn((entity, multiverse_id));
        }
    }
}

