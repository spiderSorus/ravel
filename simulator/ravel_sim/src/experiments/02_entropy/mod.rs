use bevy::prelude::*;
use crate::resources::SimulationState;
use crate::experiments::ExperimentState;
use ravel_ecs::Spawner;

pub struct EntropyPlugin;

impl Plugin for EntropyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ExperimentState::Entropy), setup)
           .add_systems(Update, tick.run_if(in_state(ExperimentState::Entropy)));
    }
}

fn setup(mut sim: ResMut<SimulationState>) {
    sim.universe.entities.clear();
    sim.universe.global_chord = [0; 64];
    sim.current_resolution = 32; 
    sim.timer = Timer::from_seconds(0.1, TimerMode::Repeating); 
}

fn tick(time: Res<Time>, mut sim: ResMut<SimulationState>) {
    sim.timer.tick(time.delta());
    if sim.timer.finished() {
        // Randomly remove some entities
        if sim.universe.entities.len() > 1000 {
             sim.universe.entities.truncate(500);
        }
        
        // Spawn high res noise
        for _ in 0..100 {
             let plant = Spawner::spawn_plant(32);
             sim.universe.spawn(plant);
        }
    }
    sim.universe.sample_graph();
}
