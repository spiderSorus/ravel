use bevy::prelude::*;

use crate::experiments::ExperimentState;

pub struct ShowRectangleExperiment;

impl Plugin for ShowRectangleExperiment {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(ExperimentState::ShowRectangle), setup_rectangle)
            .add_systems(OnExit(ExperimentState::ShowRectangle), cleanup_rectangle)
            .add_systems(Update, toggle_rectangle.run_if(in_state(ExperimentState::ShowRectangle)));
    }
}

#[derive(Component)]
struct Rectangle;

fn setup_rectangle(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(200.0, 200.0)),
                ..default()
            },
            ..default()
        },
        Rectangle,
    ));
}

fn cleanup_rectangle(mut commands: Commands, query: Query<Entity, With<Rectangle>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn toggle_rectangle(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Visibility, With<Rectangle>>) {
    if keys.just_pressed(KeyCode::KeyF) {
        for mut visibility in query.iter_mut() {
            *visibility = if *visibility == Visibility::Hidden { Visibility::Visible } else { Visibility::Hidden };
        }
    }
}
