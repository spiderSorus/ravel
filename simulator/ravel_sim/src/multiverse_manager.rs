use bevy::prelude::*;
use ravel_ecs::{MultiverseId, MultiverseState, MultiverseLibrary, ActiveMultiverse, MultiverseConfig, calculate_chord_for_multiverse};

/// Load a multiverse - despawn all entities from any previous multiverse and initialize the new one
pub fn load_multiverse(
    multiverse_id: MultiverseId,
    mut commands: Commands,
    world: &World,
    library: Res<MultiverseLibrary>,
    mut active: ResMut<ActiveMultiverse>,
) {
    // If there's an active multiverse, unload it first
    if let Some(current_id) = active.id {
        unload_multiverse(current_id, &mut commands, world);
    }

    // Get the config
    if let Some(config) = library.get(multiverse_id) {
        // Create new multiverse state
        let mut state = MultiverseState::new(config.clone());
        
        // Calculate initial chord (should be empty since we just cleared)
        state.global_chord = calculate_chord_for_multiverse(world, multiverse_id);
        
        // Set as active
        active.set_active(multiverse_id);
        
        // Store the state as a resource
        commands.insert_resource(state);
    }
}

/// Unload a multiverse - despawn all its entities and clear its state
pub fn unload_multiverse(
    multiverse_id: MultiverseId,
    commands: &mut Commands,
    world: &World,
) {
    // Query for all entities belonging to this multiverse
    let entities_to_despawn: Vec<Entity> = world
        .query_filtered::<Entity, With<ravel_ecs::MultiverseId>>()
        .iter(world)
        .filter(|entity| {
            if let Ok(mv_id) = world.entity(*entity).get::<ravel_ecs::MultiverseId>() {
                *mv_id == multiverse_id
            } else {
                false
            }
        })
        .collect();

    // Despawn all entities
    for entity in entities_to_despawn {
        commands.entity(entity).despawn();
    }
}

/// Helper to spawn an entity with multiverse tagging
pub fn spawn_with_multiverse(
    commands: &mut Commands,
    multiverse_id: MultiverseId,
    entity_bundle: impl Bundle,
) {
    commands.spawn((
        entity_bundle,
        multiverse_id,
    ));
}

/// Sample the global chord for the active multiverse
pub fn sample_active_multiverse_chord(
    world: &World,
    mut state: ResMut<MultiverseState>,
) {
    // Calculate aggregate chord from all entities in this multiverse
    state.global_chord = calculate_chord_for_multiverse(world, state.config.id);
}
