use bevy::prelude::*;

#[path = "01_genesis/mod.rs"]
pub mod genesis;
#[path = "02_entropy/mod.rs"]
pub mod entropy;
#[path = "03_equilibrium/mod.rs"]
pub mod equilibrium;
#[path = "04_civilization/mod.rs"]
pub mod civilization;
#[path = "05_show_a_map/mod.rs"]
pub mod show_a_map;
#[path = "06_population_map/mod.rs"]
pub mod population_map;

use genesis::GenesisPlugin;
use entropy::EntropyPlugin;
use equilibrium::EquilibriumPlugin;
use civilization::CivilizationPlugin;
use show_a_map::ShowAMapExperiment;
use population_map::PopulationMapPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ExperimentState {
    #[default]
    None,
    Genesis,
    Entropy,
    Equilibrium,
    Civilization,
    ShowAMap,
    PopulationMap,
}

pub struct Experiment {
    pub id: usize,
    pub name: &'static str,
    pub description: &'static str,
    pub state: ExperimentState,
}

pub const EXPERIMENTS: &[Experiment] = &[
    Experiment {
        id: 1,
        name: "Genesis",
        description: "The universe springs from a single point.",
        state: ExperimentState::Genesis,
    },
    Experiment {
        id: 2,
        name: "Entropy",
        description: "All things tend toward chaos.",
        state: ExperimentState::Entropy,
    },
    Experiment {
        id: 3,
        name: "Equilibrium",
        description: "A state of balance.",
        state: ExperimentState::Equilibrium,
    },
    Experiment {
        id: 4,
        name: "Civilization",
        description: "Order emerges from chaos.",
        state: ExperimentState::Civilization,
    },
    Experiment {
        id: 5,
        name: "Show a Map",
        description: "Shows a map.",
        state: ExperimentState::ShowAMap,
    },
    Experiment {
        id: 6,
        name: "Population Map",
        description: "Spawns a population on the world map.",
        state: ExperimentState::PopulationMap,
    },
];

pub struct ExperimentsPlugin;

impl Plugin for ExperimentsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ExperimentState>()
            .add_plugins((
                GenesisPlugin,
                EntropyPlugin,
                EquilibriumPlugin,
                CivilizationPlugin,
                ShowAMapExperiment,
                PopulationMapPlugin,
            ));
    }
}
