// The Multiverse architecture is designed for modularity and extensibility,
// allowing for dynamic rule sets and behaviors that govern the simulation.
// This approach enables easy creation of mods and custom experiments.

// --- 1. Core Concepts ---
//
// a. Multiverse: A container for entities and a specific set of rules. Each
//    multiverse is a self-contained simulation environment.
//
// b. Rule: A Bevy system that defines a behavior or interaction within a
//    multiverse. Rules are applied to entities that meet their component
//    requirements.
//
// c. System: A broader concept that can be composed of multiple rules and
//    component definitions. Systems can be documented in TOML files for clarity
//    and moddability.
//
// d. Modifier: A special type of rule that can alter the environment or other
//    entities, such as a "god system" that introduces a powerful entity and
//    changes the simulation's dynamics.

use bevy::prelude::*;
use crate::ravel::Ravel;
use std::any::TypeId;
use downcast_rs::{impl_downcast, Downcast};

// --- 2. Rule Trait ---
// The `Rule` trait defines the interface for all behaviors in the simulation.
// Each rule is a system that can be dynamically added to a multiverse.
pub trait Rule: Downcast + Send + Sync {
    fn apply(&self, world: &mut World);
}
impl_downcast!(Rule);

// --- 3. Multiverse Struct ---
// The `Multiverse` now holds a Bevy `World` and a list of rules. This allows
// each multiverse to manage its own entities, components, and systems
// independently.
#[derive(Resource)]
pub struct Multiverse {
    pub world: World,
    rules: Vec<Box<dyn Rule>>,
    pub required_components: Vec<TypeId>,
    pub global_chord: [i128; 64],
}

impl Multiverse {
    pub fn new(required_components: Vec<TypeId>) -> Self {
        Self {
            world: World::new(),
            rules: Vec::new(),
            required_components,
            global_chord: [0; 64],
        }
    }

    // Adds a new rule to the multiverse's behavior set.
    pub fn add_rule<T: Rule + 'static>(&mut self, rule: T) {
        self.rules.push(Box::new(rule));
    }

    // Spawns an entity, but only if it has the required components.
    pub fn spawn(&mut self, entity: impl Bundle) {
        // This is a simplified check. A more robust implementation would
        // involve inspecting the bundle's component types.
        // For now, we assume the spawner respects the multiverse rules.
        self.world.spawn(entity);
    }

    // Applies all registered rules to the multiverse's world.
    pub fn update(&mut self) {
        for rule in &self.rules {
            rule.apply(&mut self.world);
        }
    }

    // A specific system, now implemented as a method that could be called by a rule.
    pub fn sample_graph(&mut self) {
        let mut aggregate = [0i128; 64];
        let mut query = self.world.query::<&Ravel>();
        for ravel in query.iter(&self.world) {
            for i in 0..64 {
                aggregate[i] += ravel.chord[i] as i128;
            }
        }
        self.global_chord = aggregate;
    }
}
