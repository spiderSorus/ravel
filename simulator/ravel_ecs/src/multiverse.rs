use crate::ravel::Ravel;

// --- 4. THE MULTIVERSE (The Sensor) ---
pub struct Multiverse {
    pub entities: Vec<Ravel>,
    pub global_chord: [i128; 64],
}

impl Multiverse {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            global_chord: [0; 64],
        }
    }

    pub fn spawn(&mut self, entity: Ravel) {
        self.entities.push(entity);
    }

    pub fn sample_graph(&mut self) {
        let mut aggregate = [0i128; 64];
        for entity in &self.entities {
            for i in 0..64 {
                aggregate[i] += entity.chord[i] as i128;
            }
        }
        self.global_chord = aggregate;
    }
}
