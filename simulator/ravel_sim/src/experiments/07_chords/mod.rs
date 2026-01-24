use bevy::prelude::*;
use crate::resources::SimulationState;
use crate::experiments::ExperimentState;
use crate::map_resources::{LandMask, LandMaskLoaded};
use ravel_ecs::{Spawner, RavelType, MultiverseId, MultiverseState};
use rand::Rng;

pub struct ChordsPlugin;

impl Plugin for ChordsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ExperimentState::Chords), setup)
           .add_systems(Update, (
               tick.run_if(in_state(ExperimentState::Chords).and_then(resource_exists::<LandMaskLoaded>)),
           ));
    }
}

fn setup(
    mut sim: ResMut<SimulationState>,
    mut multiverse_state: ResMut<MultiverseState>,
) {
    multiverse_state.reset_chord();
    sim.step_counter = 0;
    sim.timer = Timer::from_seconds(0.05, TimerMode::Repeating);
}

fn tick(
    time: Res<Time>,
    mut commands: Commands,
    mut sim: ResMut<SimulationState>,
    land_mask: Res<LandMask>,
) {
    sim.timer.tick(time.delta());
    if sim.timer.finished() {
        let mut rng = rand::thread_rng();
        let mut spawned = 0;
        let mut attempts = 0;
        let multiverse_id = MultiverseId(6); // Chords multiverse ID

        while spawned < 20 && attempts < 200 {
            attempts += 1;
            let lon = rng.gen_range(-180.0..180.0);
            let lat = rng.gen_range(-60.0..85.0);

            if land_mask.is_land(lon, lat) {
                let mut entity = Spawner::spawn(RavelType::Plant, None);
                entity.pos = ravel_ecs::Pos::new(lon, lat);
                commands.spawn((entity, multiverse_id));
                spawned += 1;
            }
        }
    }
}
