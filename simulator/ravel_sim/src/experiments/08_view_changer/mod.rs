use bevy::prelude::*;
use crate::world_view::WorldView;
use crate::experiments::ExperimentState;

pub struct ViewChangerPlugin;

#[derive(Component)]
struct PlayerHud;

impl Plugin for ViewChangerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ExperimentState::ViewChanger), setup)
           .add_systems(Update, (
               toggle_view.run_if(in_state(ExperimentState::ViewChanger)),
               manage_player_hud.run_if(in_state(ExperimentState::ViewChanger)),
           ))
           .add_systems(OnExit(ExperimentState::ViewChanger), cleanup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("FiraSans-Bold.ttf");
    
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayerHud,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "PLAYER VIEW",
            TextStyle {
                font,
                font_size: 60.0,
                color: Color::WHITE,
            },
        ));
    });
}

fn toggle_view(
    keys: Res<ButtonInput<KeyCode>>,
    current_view: Res<State<WorldView>>,
    mut next_view: ResMut<NextState<WorldView>>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        match *current_view.get() {
            WorldView::Player => next_view.set(WorldView::Standard),
            _ => next_view.set(WorldView::Player),
        }
    }
}

fn manage_player_hud(
    world_view: Res<State<WorldView>>,
    mut query: Query<&mut Visibility, With<PlayerHud>>,
) {
    let should_be_visible = *world_view.get() == WorldView::Player;
    let new_vis = if should_be_visible { Visibility::Visible } else { Visibility::Hidden };
    
    for mut vis in query.iter_mut() {
        if *vis != new_vis {
            *vis = new_vis;
        }
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<PlayerHud>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
