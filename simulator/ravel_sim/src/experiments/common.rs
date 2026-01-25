use bevy::prelude::*;
use ravel_ecs::{MultiverseId, MultiverseState, MultiverseConfig, ActiveMultiverse, ravel::Ravel, ravel::Pos, ravel::Locomotion};
use crate::resources::SimulationState;
use crate::map_resources::LandMask;
use rand::Rng;

/// Standard setup for an experiment that uses a Multiverse.
/// - Sets up MultiverseState with the given config.
/// - Resets chord.
/// - Sets ActiveMultiverse.
/// - Resets SimulationState timer/counters.
pub fn setup_multiverse_experiment(
    config: MultiverseConfig,
    sim_settings: (u8, f32), // resolution, timer_seconds
    mut sim: &mut SimulationState,
    mut multiverse_state: &mut MultiverseState,
    mut active_multiverse: &mut ActiveMultiverse,
) {
    let id = config.id;
    
    // Set Multiverse State
    // We update the config ID and name if provided, or just overwrite the whole state
    *multiverse_state = MultiverseState::new(config);
    multiverse_state.reset_chord();
    
    // Set Active Multiverse
    active_multiverse.set_active(id);
    
    // Set Simulation Settings
    sim.current_resolution = sim_settings.0;
    sim.step_counter = 0;
    sim.timer = Timer::from_seconds(sim_settings.1, TimerMode::Repeating);
}

/// Spawns Ravel entities with customizable generation logic.
pub fn spawn_ravels(
    commands: &mut Commands,
    multiverse_id: MultiverseId,
    count: usize,
    chord_generator: impl Fn(&mut rand::rngs::ThreadRng) -> [i32; 64],
) {
    let mut rng = rand::thread_rng();
    
    for _ in 0..count {
        let pos = Pos {
            x: rng.gen_range(-180.0..180.0),
            y: rng.gen_range(-90.0..90.0),
        };
        
        let chord = chord_generator(&mut rng);
        
        let ravel = Ravel::new(
            true, // base: Taijitu (true for object)
            Locomotion::Passive,
            rng.gen_range(0.0..360.0), // degree
            pos,
            chord,
        );
        
        commands.spawn((ravel, multiverse_id));
    }
}

/// Spawns Ravel entities only on land.
pub fn spawn_ravels_on_land(
    commands: &mut Commands,
    multiverse_id: MultiverseId,
    count: usize,
    chord_generator: impl Fn(&mut rand::rngs::ThreadRng) -> [i32; 64],
    land_mask: &LandMask,
) {
    let mut rng = rand::thread_rng();
    let mut spawned = 0;
    let mut attempts = 0;
    let max_attempts = count * 100; // Allow many attempts

    while spawned < count && attempts < max_attempts {
        attempts += 1;
        let lon = rng.gen_range(-180.0..180.0);
        let lat = rng.gen_range(-85.0..85.0);

        if land_mask.is_land(lon, lat) {
            let pos = Pos::new(lon, lat);
            let chord = chord_generator(&mut rng);
            
            let ravel = Ravel::new(
                true, // base: Taijitu (true for object)
                Locomotion::Passive,
                rng.gen_range(0.0..360.0), // degree
                pos,
                chord,
            );
            
            commands.spawn((ravel, multiverse_id));
            spawned += 1;
        }
    }
    
    if spawned < count {
        warn!("Could only spawn {}/{} entities on land after {} attempts.", spawned, count, attempts);
    }
}
