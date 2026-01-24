use bevy::prelude::*;
use crate::resources::SimulationState;
use crate::experiments::ExperimentState;
use ravel_ecs::{RavelObject, Taijitu, Locomotion, Spawner};
use rand::Rng;

pub struct CivilizationPlugin;

impl Plugin for CivilizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ExperimentState::Civilization), setup)
           .add_systems(Update, tick.run_if(in_state(ExperimentState::Civilization)));
    }
}

fn setup(mut sim: ResMut<SimulationState>) {
    sim.universe.entities.clear();
    sim.universe.global_chord = [0; 64];
    sim.current_resolution = 64; // "Same epoch of 64"
    sim.timer = Timer::from_seconds(0.01, TimerMode::Repeating);
    sim.step_counter = 0;

    let mut rng = rand::thread_rng();

    // Spawn 20 fully formed (Res 64)
    for _ in 0..20 {
        sim.universe.spawn(Spawner::spawn_plant(64));
    }

    // Spawn 980 random entities (8 tiers of 8: 8, 16, 24, 32, 40, 48, 56, 64)
    for _ in 0..980 {
        let tier = rng.gen_range(1..=8);
        let res = tier * 8;
        sim.universe.spawn(Spawner::spawn_plant(res));
    }
    
    sim.universe.sample_graph();
}

fn tick(time: Res<Time>, mut sim: ResMut<SimulationState>) {
    sim.timer.tick(time.delta());
    
    // We only update on tick completion
    if sim.timer.finished() {
        let mut rng = rand::thread_rng();
        let count = sim.universe.entities.len();
        
        // Target 1,000,000 entities
        if count < 1_000_000 {
            // Spawn batch size (e.g. 100 per tick to reach 1M in ~100s)
            let batch_size = 100; 
            
            for _ in 0..batch_size {
                if sim.universe.entities.len() >= 1_000_000 { break; }
                
                // Random resolution from 8 tiers
                let tier = rng.gen_range(1..=8);
                let res = tier * 8;
                
                let entity = spawn_balancer(res, &sim.universe.global_chord);
                sim.universe.spawn(entity);
            }
        }
        
        // Sample every tick? Or every frame?
        // Main loop calls sample_graph() in Update? 
        // No, main.rs doesn't call sample_graph automatically for us in shared systems.
        // It's called inside the specific tick systems in previous experiments.
        // So we must call it here.
        sim.universe.sample_graph();
    }
}

fn spawn_balancer(resolution: u8, global_chord: &[i32; 64]) -> RavelObject {
    let mut rng = rand::thread_rng();
    let mut chord = [0; 64];

    for i in 0..resolution as usize {
        if i >= 64 { break; }
        
        // Balancing logic: 
        // If global is positive, we want negative.
        // If global is negative, we want positive.
        let current_val = global_chord[i];
        
        // 50% chance to have a value at this position (like Spawner)
        if rng.gen_bool(0.5) {
             if current_val > 0 {
                 chord[i] = rng.gen_range(-100..0);
             } else if current_val < 0 {
                 chord[i] = rng.gen_range(0..100);
             } else {
                 chord[i] = rng.gen_range(-100..100);
             }
        }
    }

    RavelObject::new(
        Taijitu::Object, 
        Locomotion::Active, // Civilization implies Active?
        resolution, 
        chord
    )
}
