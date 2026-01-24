use bevy::prelude::*;

#[path = "01_genesis/mod.rs"]
pub mod genesis;
#[path = "02_entropy/mod.rs"]
pub mod entropy;
#[path = "03_equilibrium/mod.rs"]
pub mod equilibrium;
#[path = "04_civilization/mod.rs"]
pub mod civilization;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ExperimentState {
    #[default]
    None,
    Genesis,
    Entropy,
    Equilibrium,
    Civilization,
}

pub struct ExperimentInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub id: usize,
    pub state: ExperimentState,
}

pub const EXPERIMENTS: &[ExperimentInfo] = &[
    ExperimentInfo { 
        name: "Genesis", 
        description: "Evolution from simplicity to complexity",
        id: 1, 
        state: ExperimentState::Genesis 
    },
    ExperimentInfo { 
        name: "Entropy", 
        description: "High energy chaotic fluctuations",
        id: 2, 
        state: ExperimentState::Entropy 
    },
    ExperimentInfo { 
        name: "Equilibrium", 
        description: "Balanced steady state",
        id: 3, 
        state: ExperimentState::Equilibrium 
    },
    ExperimentInfo { 
        name: "Civilization", 
        description: "Growth to 1,000,000 with auto-balancing",
        id: 4, 
        state: ExperimentState::Civilization 
    },
];

pub struct ExperimentsPlugin;

impl Plugin for ExperimentsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ExperimentState>()
           .add_plugins((
               genesis::GenesisPlugin,
               entropy::EntropyPlugin,
               equilibrium::EquilibriumPlugin,
               civilization::CivilizationPlugin,
           ));
    }
}
