use bevy::prelude::*;
use bevy_svg::prelude::*;

use crate::experiments::ExperimentState;

pub struct ShowAMapExperiment;

impl Plugin for ShowAMapExperiment {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(ExperimentState::ShowAMap), setup_map)
            .add_systems(OnExit(ExperimentState::ShowAMap), cleanup_map)
            .add_systems(Update, toggle_map.run_if(in_state(ExperimentState::ShowAMap)));
    }
}

#[derive(Component)]
struct Map;

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>, windows: Query<&Window>) {
    let window = windows.single();
    let texture = asset_server.load("map.png");

    let image_width = 2000.0;
    let image_height = 1478.0;
    let image_aspect = image_width / image_height;
    let window_aspect = window.width() / window.height();

    let size = if window_aspect > image_aspect {
        Vec2::new(window.height() * image_aspect, window.height())
    } else {
        Vec2::new(window.width(), window.width() / image_aspect)
    };

    commands.spawn((
        SpriteBundle {
            texture,
            sprite: Sprite {
                custom_size: Some(size),
                ..default()
            },
            ..default()
        },
        Map,
    ));
}

fn cleanup_map(mut commands: Commands, query: Query<Entity, With<Map>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn toggle_map(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Visibility, With<Map>>) {
    if keys.just_pressed(KeyCode::KeyE) {
        for mut visibility in query.iter_mut() {
            *visibility = if *visibility == Visibility::Hidden { Visibility::Visible } else { Visibility::Hidden };
        }
    }
}
