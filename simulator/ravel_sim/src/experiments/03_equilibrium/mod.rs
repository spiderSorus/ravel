use bevy::prelude::*;
use crate::resources::SimulationState;
use crate::experiments::ExperimentState;
use ravel_ecs::Spawner;

pub struct EquilibriumPlugin;

impl Plugin for EquilibriumPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ExperimentState::Equilibrium), setup)
           .add_systems(Update, tick.run_if(in_state(ExperimentState::Equilibrium)));
    }
}

fn setup(mut sim: ResMut<SimulationState>) {
    sim.universe.entities.clear();
    sim.universe.global_chord = [0; 64];
    sim.current_resolution = 8; 
    sim.timer = Timer::from_seconds(2.0, TimerMode::Repeating); 

    // Fill initial state
    for _ in 0..200 {
        sim.universe.spawn(Spawner::spawn_plant(8));
    }
}

fn tick(time: Res<Time>, mut sim: ResMut<SimulationState>) {
    sim.timer.tick(time.delta());
    if sim.timer.finished() {
        // Maintain population count
        let target_pop = 200;
        let current_pop = sim.universe.entities.len();
        
        if current_pop < target_pop {
            for _ in 0..(target_pop - current_pop) {
                sim.universe.spawn(Spawner::spawn_plant(8));
            }
        } else if current_pop > target_pop {
            sim.universe.entities.truncate(target_pop);
        }
    }
    sim.universe.sample_graph();
}
