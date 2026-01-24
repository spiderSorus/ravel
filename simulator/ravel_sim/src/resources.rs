use bevy::prelude::*;
use ravel_ecs::Multiverse;

#[derive(Resource)]
pub struct SimulationState {
    pub multiverse: Multiverse,
    pub timer: Timer,
    pub current_resolution: u8, // Note: This is now legacy, but kept for experiments that use it.
    pub step_counter: u32,
}
