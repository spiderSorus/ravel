use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum WorldView {
    #[default]
    None,
    Standard, // All mechanics
    MapOnly,  
    MandalaOnly,
    Player,   // HUD only
}

pub struct WorldViewPlugin;

impl Plugin for WorldViewPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<WorldView>();
        
        // Default to Standard when entering Simulation state
        // (This can be overridden programmatically)
        app.add_systems(OnEnter(crate::AppState::Simulation), set_default_view);
    }
}

fn set_default_view(mut next_view: ResMut<NextState<WorldView>>) {
    next_view.set(WorldView::Standard);
}
