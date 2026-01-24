use bevy::prelude::*;
use crate::resources::SimulationState;
use crate::experiments::ExperimentState;
use ravel_ecs::{Spawner, RavelType, MultiverseState, MultiverseId};

pub struct GenesisPlugin;

impl Plugin for GenesisPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ExperimentState::Genesis), setup)
           .add_systems(Update, tick.run_if(in_state(ExperimentState::Genesis)));
    }
}

fn setup(
    mut commands: Commands,
    mut sim: ResMut<SimulationState>,
    mut multiverse_state: ResMut<MultiverseState>,
) {
    multiverse_state.reset_chord();
    sim.current_resolution = 1;
    sim.step_counter = 0;
    sim.timer = Timer::from_seconds(1.0, TimerMode::Repeating);
    
    let multiverse_id = MultiverseId(1);
    
    for _ in 0..50 {
        let plant = Spawner::spawn(RavelType::Plant, None);
        commands.spawn((plant, multiverse_id));
    }
}

fn tick(
    time: Res<Time>,
    mut commands: Commands,
    mut sim: ResMut<SimulationState>,
    entities_query: Query<Entity, With<ravel_ecs::MultiverseId>>,
) {
    sim.timer.tick(time.delta());
    if sim.timer.finished() {
        if sim.current_resolution < 64 {
            sim.current_resolution *= 2;
            sim.step_counter = 0;
        } else {
            sim.step_counter += 1;
            if sim.step_counter > 5 {
                sim.current_resolution = 1;
                sim.step_counter = 0;
                
                let multiverse_id = MultiverseId(1);
                // Despawn entities from Genesis multiverse
                let entities_to_despawn: Vec<_> = entities_query
                    .iter()
                    .collect();
                
                for entity in entities_to_despawn {
                    commands.entity(entity).despawn();
                }
            }
        }
        
        let multiverse_id = MultiverseId(1);
        for _ in 0..20 {
            let plant = Spawner::spawn(RavelType::Plant, None);
            commands.spawn((plant, multiverse_id));
            let plant = Spawner::spawn(RavelType::Plant, None);
            commands.spawn((plant, multiverse_id));
        }
    }
}
