use rand::Rng;
use crate::ravel::{Ravel, Locomotion, Pos};

// --- 5. THE SPAWNER ---
pub struct Spawner;

impl Spawner {
    pub fn spawn_plant(epoch_resolution: u8) -> Ravel {
        let mut rng = rand::thread_rng();
        let mut chord = [0; 64];

        for i in 0..epoch_resolution as usize {
            if i >= 64 { break; }
            if rng.gen_bool(0.5) {
                chord[i] = rng.gen_range(-100..100);
            }
        }

        let pos = Pos {
            x: rng.gen_range(-180.0..180.0),
            y: rng.gen_range(-90.0..90.0),
        };

        Ravel::new(
            true, // Taijitu::Object
            Locomotion::Passive,
            epoch_resolution,
            pos,
            chord
        )
    }
}
