use rand::Rng;
use crate::ravel::{Ravel, Locomotion, Pos};

// --- 5. THE SPAWNER ---
#[derive(Debug, Clone, Copy)]
pub enum RavelType {
    Plant,
    Balancer,
}

pub struct Spawner;

impl Spawner {
    pub fn spawn(ravel_type: RavelType, global_chord: Option<&[i128; 64]>) -> Ravel {
        match ravel_type {
            RavelType::Plant => Self::spawn_plant(),
            RavelType::Balancer => {
                if let Some(chord) = global_chord {
                    Self::spawn_balancer(chord)
                } else {
                    Self::spawn_plant()
                }
            }
        }
    }

    pub fn spawn_plant() -> Ravel {
        let mut rng = rand::thread_rng();
        let mut chord = [0; 64];

        for i in 0..64 {
            if rng.gen_bool(0.5) {
                chord[i] = rng.r#gen::<i32>();
            }
        }
        
        let degree = rng.gen_range(0.0..360.0);

        let pos = Pos {
            x: rng.gen_range(-180.0..180.0),
            y: rng.gen_range(-90.0..90.0),
        };

        Ravel::new(
            true, // Taijitu::Object
            Locomotion::Passive,
            degree,
            pos,
            chord
        )
    }

    pub fn spawn_balancer(global_chord: &[i128; 64]) -> Ravel {
        let mut rng = rand::thread_rng();
        let mut chord = [0; 64];

        for i in 0..64 {
            let current_val = global_chord[i];
            if rng.gen_bool(0.5) {
                if current_val > 0 {
                    chord[i] = rng.gen_range(i32::MIN..0);
                } else if current_val < 0 {
                    chord[i] = rng.gen_range(0..i32::MAX);
                } else {
                    chord[i] = rng.r#gen::<i32>();
                }
            }
        }

        let degree = rng.gen_range(0.0..360.0);

        Ravel::new(
            true, // Taijitu::Object
            Locomotion::Active,
            degree,
            Pos { x: 0.0, y: 0.0 },
            chord
        )
    }
}
