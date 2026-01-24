// The Multiverse architecture is designed for modularity and extensibility,
// allowing for dynamic rule sets and behaviors that govern the simulation.
// This approach enables easy creation of mods and custom experiments.

// --- Architecture ---
// Each Multiverse is a configuration and state object, not a container.
// All entities live in a single Bevy World and are tagged with MultiverseId.
// When a multiverse is loaded, entities are spawned with its genesis rules.
// When a multiverse is unloaded, all its entities are despawned.

use bevy::prelude::*;
use crate::ravel::Ravel;
use std::any::TypeId;

// --- MultiverseId Component ---
// Tags entities with their owning multiverse
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MultiverseId(pub u32);

// --- Multiverse Struct ---
// Configuration and state for a multiverse (not a World owner)
#[derive(Clone)]
pub struct MultiverseConfig {
    pub id: MultiverseId,
    pub name: String,
    pub required_components: Vec<TypeId>,
}

#[derive(Resource, Clone)]
pub struct MultiverseState {
    pub config: MultiverseConfig,
    pub global_chord: [i128; 64],
}

impl MultiverseState {
    pub fn new(config: MultiverseConfig) -> Self {
        Self {
            config,
            global_chord: [0; 64],
        }
    }

    pub fn reset_chord(&mut self) {
        self.global_chord = [0; 64];
    }

    pub fn clear(&mut self) {
        self.reset_chord();
    }
}

// --- Active Multiverse Tracking ---
#[derive(Resource, Default)]
pub struct ActiveMultiverse {
    pub id: Option<MultiverseId>,
}

impl ActiveMultiverse {
    pub fn is_loaded(&self) -> bool {
        self.id.is_some()
    }

    pub fn set_active(&mut self, id: MultiverseId) {
        self.id = Some(id);
    }

    pub fn clear_active(&mut self) {
        self.id = None;
    }
}

// --- Multiverse Library ---
// Stores all available multiverse configurations
#[derive(Resource, Default)]
pub struct MultiverseLibrary {
    multiverses: Vec<MultiverseConfig>,
}

impl MultiverseLibrary {
    pub fn new() -> Self {
        Self {
            multiverses: Vec::new(),
        }
    }

    pub fn register(&mut self, config: MultiverseConfig) {
        self.multiverses.push(config);
    }

    pub fn get(&self, id: MultiverseId) -> Option<&MultiverseConfig> {
        self.multiverses.iter().find(|m| m.id == id)
    }

    pub fn list(&self) -> &[MultiverseConfig] {
        &self.multiverses
    }
}

// --- Helper function to calculate aggregate chord ---
pub fn calculate_chord_for_multiverse(world: &World, multiverse_id: MultiverseId) -> [i128; 64] {
    let mut aggregate = [0i128; 64];
    let mut query = world.query::<(&MultiverseId, &Ravel)>();
    for (id, ravel) in query.iter(world) {
        if *id == multiverse_id {
            for i in 0..64 {
                aggregate[i] += ravel.chord[i] as i128;
            }
        }
    }
    aggregate
}

// --- Helper function to get all ravels in a multiverse ---
pub fn get_ravels_for_multiverse(world: &World, multiverse_id: MultiverseId) -> Vec<Ravel> {
    let mut ravels = Vec::new();
    let mut query = world.query::<(&MultiverseId, &Ravel)>();
    for (id, ravel) in query.iter(world) {
        if *id == multiverse_id {
            ravels.push(ravel.clone());
        }
    }
    ravels
}
