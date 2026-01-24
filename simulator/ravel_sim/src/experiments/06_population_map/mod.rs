use bevy::prelude::*;
use crate::resources::SimulationState;
use crate::experiments::ExperimentState;
use crate::map_resources::{LandMask, LandMaskLoaded};
use ravel_ecs::{Spawner, RavelType};
use rand::Rng;

pub struct PopulationMapPlugin;

impl Plugin for PopulationMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ExperimentState::PopulationMap), setup)
           .add_systems(Update, (
               tick.run_if(in_state(ExperimentState::PopulationMap).and_then(resource_exists::<LandMaskLoaded>)),
           ));
    }
}

fn setup(mut sim: ResMut<SimulationState>) {
    sim.universe.entities.clear();
    sim.universe.global_chord = [0; 64];
    sim.current_resolution = 1;
    sim.step_counter = 0;
    sim.timer = Timer::from_seconds(0.05, TimerMode::Repeating);
}

fn tick(
    time: Res<Time>, 
    mut sim: ResMut<SimulationState>, 
    land_mask: Res<LandMask>
) {
    sim.timer.tick(time.delta());
    if sim.timer.finished() {
        if sim.current_resolution < 64 {
            sim.current_resolution += 1;
        } else {
            sim.current_resolution = 1;
        }
        
        let mut rng = rand::thread_rng();
        let mut spawned = 0;
        let mut attempts = 0;

        while spawned < 20 && attempts < 200 {
            attempts += 1;
            let lon = rng.gen_range(-180.0..180.0);
            let lat = rng.gen_range(-60.0..85.0); // Focus on habitable latitudes

            if land_mask.is_land(lon, lat) {
                let mut entity = Spawner::spawn(RavelType::Plant, None);
                entity.pos = ravel_ecs::Pos::new(lon, lat);
                sim.universe.spawn(entity);
                spawned += 1;
            }
        }
    }
    sim.universe.sample_graph();
}
