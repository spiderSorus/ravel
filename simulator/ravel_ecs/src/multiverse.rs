use crate::ravel::Ravel;

// --- 4. THE MULTIVERSE (The Sensor) ---
pub struct Multiverse {
    pub entities: Vec<Ravel>,
    pub global_chord: [i32; 64],
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
