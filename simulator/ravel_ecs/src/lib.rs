pub mod iching;
use rand::Rng;

// --- 1. THE BINARY BASE (Taijitu) ---
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Taijitu {
    Action = 0, // Yang / Verb
    Object = 1, // Yin / Noun
}

// --- 2. THE LOCOMOTION LAYER ---
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Locomotion {
    Passive = 0,
    Active = 1,
}

// --- 3. THE ATOM (RavelObject) ---
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RavelObject {
    pub base: Taijitu,
    pub locomotion: Locomotion,
    pub resolution: u8, 
    pub chord: [i32; 64],
}

impl RavelObject {
    pub fn new(base: Taijitu, locomotion: Locomotion, resolution: u8, chord: [i32; 64]) -> Self {
        Self { base, locomotion, resolution, chord }
    }
}

// --- 4. THE MULTIVERSE (The Sensor) ---
pub struct Multiverse {
    pub entities: Vec<RavelObject>,
    pub global_chord: [i32; 64],
}

impl Multiverse {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            global_chord: [0; 64],
        }
    }

    pub fn spawn(&mut self, entity: RavelObject) {
        self.entities.push(entity);
    }

    pub fn sample_graph(&mut self) {
        let mut aggregate = [0; 64];
        for entity in &self.entities {
            let limit = entity.resolution as usize;
            for i in 0..64 {
                if i < limit {
                    aggregate[i] += entity.chord[i];
                }
            }
        }
        self.global_chord = aggregate;
    }
}

// --- 5. THE SPAWNER ---
pub struct Spawner;

impl Spawner {
    pub fn spawn_plant(epoch_resolution: u8) -> RavelObject {
        let mut rng = rand::thread_rng();
        let mut chord = [0; 64];

        for i in 0..epoch_resolution as usize {
            if i >= 64 { break; }
            if rng.gen_bool(0.5) {
                chord[i] = rng.gen_range(-100..100);
            }
        }

        RavelObject::new(
            Taijitu::Object, 
            Locomotion::Passive, 
            epoch_resolution, 
            chord
        )
    }
}
