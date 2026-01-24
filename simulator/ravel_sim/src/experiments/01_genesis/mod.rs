use bevy::prelude::*;
use crate::resources::SimulationState;
use crate::experiments::ExperimentState;
use ravel_ecs::{Spawner, RavelType};

pub struct GenesisPlugin;

impl Plugin for GenesisPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ExperimentState::Genesis), setup)
           .add_systems(Update, tick.run_if(in_state(ExperimentState::Genesis)));
    }
}

fn setup(mut sim: ResMut<SimulationState>) {
    sim.universe.entities.clear();
    sim.universe.global_chord = [0; 64];
    sim.current_resolution = 1;
    sim.step_counter = 0;
    sim.timer = Timer::from_seconds(1.0, TimerMode::Repeating);
    
    // Seed
    for _ in 0..50 {
        sim.universe.spawn(Spawner::spawn(RavelType::Plant, None));
    }
}

fn tick(time: Res<Time>, mut sim: ResMut<SimulationState>) {
    sim.timer.tick(time.delta());
    if sim.timer.finished() {
        if sim.current_resolution < 64 {
            sim.current_resolution *= 2;
            sim.step_counter = 0;
        } else {
            // We are at max resolution (64).
            // Stay here for a few epochs so Gate 1 (Index 63) is visible.
            sim.step_counter += 1;
            if sim.step_counter > 5 {
                sim.current_resolution = 1;
                sim.step_counter = 0;
                sim.universe.entities.clear();
            }
        }
        
        for _ in 0..20 {
             let plant = Spawner::spawn(RavelType::Plant, None);
             sim.universe.spawn(plant);
        }
    }
    sim.universe.sample_graph();
}
