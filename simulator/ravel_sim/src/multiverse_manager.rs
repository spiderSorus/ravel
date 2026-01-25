use bevy::prelude::*;
use ravel_ecs::{MultiverseId, MultiverseState, MultiverseLibrary, ActiveMultiverse, calculate_chord_for_multiverse};

#[allow(dead_code)]
/// Load a multiverse - despawn all entities from any previous multiverse and initialize the new one
pub fn load_multiverse(
    multiverse_id: MultiverseId,
    mut commands: Commands,
    library: Res<MultiverseLibrary>,
    mut active: ResMut<ActiveMultiverse>,
    entity_query: Query<(Entity, &MultiverseId)>,
    ravel_query: Query<(&MultiverseId, &ravel_ecs::ravel::Ravel)>,
) {
    // If there's an active multiverse, unload it first
    if let Some(current_id) = active.id {
        unload_multiverse(current_id, &mut commands, &entity_query);
    }

    // Get the config
    if let Some(config) = library.get(multiverse_id) {
        // Create new multiverse state
        let mut state = MultiverseState::new(config.clone());
        
        // Calculate initial chord (should be empty since we just cleared)
        state.global_chord = calculate_chord_for_multiverse(multiverse_id, &ravel_query);
        
        // Set as active
        active.set_active(multiverse_id);
        
        // Store the state as a resource
        commands.insert_resource(state);
    }
}

/// Unload a multiverse - despawn all its entities and clear its state
#[allow(dead_code)]
pub fn unload_multiverse(
    multiverse_id: MultiverseId,
    commands: &mut Commands,
    query: &Query<(Entity, &MultiverseId)>,
) {
    for (entity, mv_id) in query.iter() {
        if *mv_id == multiverse_id {
            commands.entity(entity).despawn();
        }
    }
}

/// Helper to spawn an entity with multiverse tagging
#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn sample_active_multiverse_chord(
    mut state: ResMut<MultiverseState>,
    query: Query<(&MultiverseId, &ravel_ecs::ravel::Ravel)>,
) {
    // Calculate aggregate chord from all entities in this multiverse
    state.global_chord = calculate_chord_for_multiverse(state.config.id, &query);
}
