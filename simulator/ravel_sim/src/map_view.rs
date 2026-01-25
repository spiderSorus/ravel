
use bevy::prelude::*;
use crate::AppState;
use crate::world_view::WorldView;

use crate::experiments::ExperimentState;

pub struct MapViewPlugin;

#[derive(Resource, Default)]
struct MapViewVisibility(pub bool);

#[derive(Component)]
pub struct MapView;

impl Plugin for MapViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapViewVisibility(true));

        app.add_systems(OnEnter(AppState::Simulation), setup_map)
           .add_systems(OnExit(AppState::Simulation), cleanup_map)
           .add_systems(OnExit(ExperimentState::Genesis), cleanup_map)
           .add_systems(OnExit(ExperimentState::Entropy), cleanup_map)
           .add_systems(OnExit(ExperimentState::Equilibrium), cleanup_map)
           .add_systems(OnExit(ExperimentState::Civilization), cleanup_map)
           .add_systems(OnExit(ExperimentState::ShowAMap), cleanup_map)
           .add_systems(OnExit(ExperimentState::PopulationMap), cleanup_map)
           .add_systems(OnExit(ExperimentState::Chords), cleanup_map)
           .add_systems(OnExit(ExperimentState::ViewChanger), cleanup_map)
           .add_systems(OnExit(ExperimentState::MultiverseA), cleanup_map)
           .add_systems(OnExit(ExperimentState::MultiverseB), cleanup_map)
           .add_systems(OnExit(ExperimentState::MultiverseSwitcher), cleanup_map)
           .add_systems(Update, (
               toggle_map_view.run_if(in_state(AppState::Simulation)),
               draw_entities_on_map.run_if(in_state(AppState::Simulation).and_then(any_with_component::<MapView>)),
               manage_map_visibility.run_if(in_state(AppState::Simulation)),
           ));
    }
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>, windows: Query<&Window>) {
    let window = windows.single();
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
            texture: asset_server.load("map.png"),
            sprite: Sprite {
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -2.0)), // Map at lowest layer
            ..default()
        },
        MapView,
    ));
}

fn cleanup_map(mut commands: Commands, query: Query<Entity, With<MapView>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn toggle_map_view(
    keys: Res<ButtonInput<KeyCode>>,
    mut visibility_res: ResMut<MapViewVisibility>,
) {
    if keys.just_pressed(KeyCode::KeyE) {
        visibility_res.0 = !visibility_res.0;
    }
}

fn manage_map_visibility(
    world_view: Res<State<WorldView>>,
    visibility_res: Res<MapViewVisibility>,
    mut query: Query<&mut Visibility, With<MapView>>,
) {
    let should_be_visible = match *world_view.get() {
        WorldView::Standard | WorldView::MapOnly => visibility_res.0,
        _ => false,
    };

    let new_vis = if should_be_visible { Visibility::Visible } else { Visibility::Hidden };
    for mut vis in query.iter_mut() {
        if *vis != new_vis {
            *vis = new_vis;
        }
    }
}

fn draw_entities_on_map(
    mut gizmos: Gizmos,
    visibility: Res<MapViewVisibility>,
    windows: Query<&Window>,
    ravels: Query<&ravel_ecs::Ravel, With<ravel_ecs::MultiverseId>>,
) {
    if !visibility.0 { return; }

    let window = windows.single();
    let image_width = 2000.0;
    let image_height = 1478.0;
    let image_aspect = image_width / image_height;
    let window_aspect = window.width() / window.height();

    let (draw_width, draw_height) = if window_aspect > image_aspect {
        (window.height() * image_aspect, window.height())
    } else {
        (window.width(), window.width() / image_aspect)
    };

    for ravel in ravels.iter() {
        let (x, y) = ravel.pos.to_mercator();

        let screen_x = (x + std::f32::consts::PI) / (2.0 * std::f32::consts::PI) * draw_width - draw_width / 2.0;
        let screen_y = -((y - std::f32::consts::PI) / (2.0 * -std::f32::consts::PI) * draw_height - draw_height / 2.0);

        let hue = ravel.degree;
        gizmos.circle_2d(Vec2::new(screen_x, screen_y), 4.0, Color::hsl(hue, 1.0, 0.5));
    }
}
