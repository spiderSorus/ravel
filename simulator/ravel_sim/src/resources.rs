use bevy::prelude::*;
use ravel_ecs::{MultiverseState, MultiverseId, MultiverseConfig};

#[derive(Resource)]
pub struct SimulationState {
    pub timer: Timer,
    pub current_resolution: u8, // Note: This is now legacy, but kept for experiments that use it.
    pub step_counter: u32,
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            current_resolution: 32,
            step_counter: 0,
        }
    }
}

/// Helper to create a multiverse config by ID
pub fn create_multiverse_config(id: u32, name: &str) -> MultiverseConfig {
    MultiverseConfig {
        id: MultiverseId(id),
        name: name.to_string(),
        required_components: vec![],
    }
}
