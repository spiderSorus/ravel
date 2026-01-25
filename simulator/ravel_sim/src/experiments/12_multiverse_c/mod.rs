use bevy::prelude::*;
use crate::resources::SimulationState;
use ravel_ecs::{MultiverseId, MultiverseState, MultiverseConfig};
use crate::experiments::common::{setup_multiverse_experiment, spawn_ravels_on_land};
use crate::map_resources::LandMask;
use rand::Rng;

pub fn spawn_multiverse_c(
    commands: &mut Commands,
    sim: &mut SimulationState,
    multiverse_state: &mut MultiverseState,
    active_multiverse: &mut ravel_ecs::ActiveMultiverse,
    land_mask: &LandMask,
) {
    let config = MultiverseConfig {
        id: MultiverseId(12),
        name: "Multiverse C".to_string(),
        required_components: vec![],
    };
    
    setup_multiverse_experiment(config, (1, 1.0), sim, multiverse_state, active_multiverse);
    
    let multiverse_id = MultiverseId(12);
    
    spawn_ravels_on_land(commands, multiverse_id, 5000, |rng| {
        let mut chord = [0; 64];
        for i in 0..64 {
             chord[i] = rng.gen::<i32>();
        }
        chord
    }, land_mask);
}

// Tick logic removed since it wasn't doing anything
