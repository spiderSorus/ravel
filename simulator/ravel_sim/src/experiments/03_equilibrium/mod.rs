use bevy::prelude::*;
use crate::resources::SimulationState;
use crate::experiments::ExperimentState;
use ravel_ecs::{Spawner, RavelType};

pub struct EquilibriumPlugin;

impl Plugin for EquilibriumPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ExperimentState::Equilibrium), setup)
           .add_systems(Update, tick.run_if(in_state(ExperimentState::Equilibrium)));
    }
}

fn setup(mut sim: ResMut<SimulationState>) {
    sim.multiverse.world.clear_entities();
    sim.multiverse.global_chord = [0; 64];
    sim.current_resolution = 8; 
    sim.timer = Timer::from_seconds(2.0, TimerMode::Repeating); 

    // Fill initial state
    for _ in 0..200 {
        sim.multiverse.spawn(Spawner::spawn(RavelType::Plant, None));
    }
}

fn tick(time: Res<Time>, mut sim: ResMut<SimulationState>) {
    sim.timer.tick(time.delta());
    if sim.timer.finished() {
        // Maintain population count
        let target_pop = 200;
        let current_pop = sim.multiverse.world.query::<&Ravel>().iter(&sim.multiverse.world).count();
        
        if current_pop < target_pop {
            for _ in 0..(target_pop - current_pop) {
                sim.multiverse.spawn(Spawner::spawn(RavelType::Plant, None));
            }
        } else if current_pop > target_pop {
            let mut query = sim.multiverse.world.query_filtered::<Entity, With<Ravel>>();
            let entities_to_remove: Vec<Entity> = query.iter(&sim.multiverse.world).take(current_pop - target_pop).collect();
            for entity in entities_to_remove {
                sim.multiverse.world.despawn(entity);
            }
        }
    }
    sim.multiverse.sample_graph();
}
