use hecs::{World, Entity};
use rand::Rng;

// --- CORE COMPONENTS ---

#[derive(Clone, Debug, PartialEq)]
pub struct Chord {
    pub values: [i32; 64],
}

impl Default for Chord {
    fn default() -> Self {
        Self { values: [0; 64] }
    }
}

impl Chord {
    pub fn new(values: [i32; 64]) -> Self {
        Self { values }
    }
}

pub struct MultiverseState; // The Tracker
pub struct Organism;        // A standard object (Food, Mob, Player)

// --- THE UNIVERSE (LIB API) ---

pub struct Universe {
    world: World,
    multiverse_entity: Entity,
}

impl Universe {
    pub fn new() -> Self {
        let mut world = World::new();
        let multiverse_entity = world.spawn((Chord::default(), MultiverseState));
        Self {
            world,
            multiverse_entity,
        }
    }

    // --- DATA ACCESS ---

    pub fn get_global_state(&self) -> [i32; 64] {
        self.world.get::<&Chord>(self.multiverse_entity)
            .map(|c| c.values)
            .unwrap_or([0; 64])
    }

    pub fn update_census(&mut self) {
        let mut aggregate = [0; 64];
        for chord in self.world.query::<&Chord>().with::<&Organism>().iter() {
            for i in 0..64 {
                aggregate[i] += chord.values[i];
            }
        }
        if let Ok(mut universe_chord) = self.world.get::<&mut Chord>(self.multiverse_entity) {
            universe_chord.values = aggregate;
        }
    }

    /// Calculates the "Hunger" of the universe (The Delta).
    /// If the world is +1000, the hunger is -1000.
    pub fn get_ecological_demand(&self) -> [i32; 64] {
        let state = self.get_global_state();
        let mut demand = [0; 64];
        for i in 0..64 {
            demand[i] = -state[i]; // Simple Inversion
        }
        demand
    }

    // --- ECOLOGY ENGINE (The Food Chain) ---

    /// Spawns a single entity whose stats are probabilistically determined
    /// by the Ecological Demand.
    ///
    /// * `intensity`: How strong the new object is (e.g., +/- 50).
    /// * `mutation_rate`: Chance (0.0-1.0) to ignore the demand and spawn random noise.
    pub fn spawn_harmonic_organism(&mut self, intensity: i32, mutation_rate: f64) -> Entity {
        let demand = self.get_ecological_demand();
        let mut rng = rand::rng();
        let mut new_genes = [0; 64];

        // We construct the genome gate by gate
        for i in 0..64 {
            // 1. Mutation Check: Sometimes nature just makes random stuff
            if rng.random_bool(mutation_rate) {
                if rng.random_bool(0.1) { // Sparse
                    new_genes[i] = rng.random_range(-intensity..intensity);
                }
                continue;
            }

            // 2. Harmonic Filling: Check if the universe is "Hungry" for this gate
            if demand[i] != 0 {
                // The probability of spawning this trait increases with Demand.
                // High Demand = High chance of spawning the antidote.
                
                // Normalize demand to a probability (Simplified logic)
                // If demand is high negative (need Yin), we spawn Negative.
                // If demand is high positive (need Yang), we spawn Positive.
                
                // We barely modify gates that are already balanced (demand ~ 0)
                if rng.random_bool(0.3) { 
                    // Spawn a gene that satisfies the demand (same sign as demand)
                    new_genes[i] = demand[i].signum() * rng.random_range(1..intensity);
                }
            }
        }

        self.world.spawn((Chord::new(new_genes), Organism))
    }
}
