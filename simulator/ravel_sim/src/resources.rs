use bevy::prelude::*;
use ravel_ecs::Multiverse;

#[derive(Resource)]
pub struct SimulationState {
    pub universe: Multiverse,
    pub timer: Timer,
    pub current_resolution: u8,
    pub step_counter: u32,
}
