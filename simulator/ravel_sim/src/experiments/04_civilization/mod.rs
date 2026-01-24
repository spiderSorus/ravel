use bevy::prelude::*;
use crate::resources::SimulationState;
use crate::experiments::ExperimentState;
use ravel_ecs::{Spawner, RavelType};

pub struct CivilizationPlugin;

impl Plugin for CivilizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ExperimentState::Civilization), setup)
           .add_systems(Update, tick.run_if(in_state(ExperimentState::Civilization)));
    }
}

fn setup(mut sim: ResMut<SimulationState>) {
    sim.multiverse.world.clear_entities();
    sim.multiverse.global_chord = [0; 64];
    sim.current_resolution = 64; // "Same epoch of 64"
    sim.timer = Timer::from_seconds(0.01, TimerMode::Repeating);
    sim.step_counter = 0;

    // Spawn 20 fully formed (Res 64)
    for _ in 0..20 {
        sim.multiverse.spawn(Spawner::spawn(RavelType::Plant, None));
    }

    // Spawn 980 random entities
    for _ in 0..980 {
        sim.multiverse.spawn(Spawner::spawn(RavelType::Plant, None));
    }
    
    sim.multiverse.sample_graph();
}

fn tick(time: Res<Time>, mut sim: ResMut<SimulationState>) {
    sim.timer.tick(time.delta());
    
    // We only update on tick completion
    if sim.timer.finished() {
        let count = sim.multiverse.world.query::<&Ravel>().iter(&sim.multiverse.world).count();
        
        // Target 1,000,000 entities
        if count < 1_000_000 {
            // Spawn batch size (e.g. 100 per tick to reach 1M in ~100s)
            let batch_size = 100; 
            
            for _ in 0..batch_size {
                if sim.multiverse.world.query::<&Ravel>().iter(&sim.multiverse.world).count() >= 1_000_000 { break; }
                
                let entity = Spawner::spawn(RavelType::Balancer, Some(&sim.multiverse.global_chord));
                sim.multiverse.spawn(entity);
            }
        }
        
        sim.multiverse.sample_graph();
    }
}

