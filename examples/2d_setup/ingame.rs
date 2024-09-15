use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::grid_coords_to_translation;

use std::collections::HashSet;

use crate::{
    WINDOW_SIZE,
    AppState,
};

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Component)]
pub struct Wall;

#[derive(Default, Component)]
pub struct Goal;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct GoalBundle {
    goal: Goal,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Resource)]
pub struct LevelWalls {
    wall_locations: HashSet<GridCoords>,
    level_width: i32,
    level_height: i32,
}

impl LevelWalls {
    fn in_wall(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.level_width 
            || grid_coords.y >= self.level_height
            || self.wall_locations.contains(grid_coords)
    }
}

const GRID_SIZE: i32 = 16;
const MAX_LEVEL_SELECTION: usize = 3;

pub fn ingame_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), With<Camera2d>>,
    ldtk_project_entities: Query<&Handle<LdtkProject>>,
) {
    let (mut camera_projection, mut camera_transform) = camera_query.single_mut();

    camera_projection.scale = 0.5;
    camera_transform.translation.x = WINDOW_SIZE.x / 4.0;
    camera_transform.translation.y = WINDOW_SIZE.y / 4.0;

    if ldtk_project_entities.is_empty() {
        commands.spawn(LdtkWorldBundle {
            ldtk_handle: asset_server.load("ldtk/tile-based-game.ldtk"),
            ..Default::default()
        });
    }
}

pub fn move_player_from_input(
    mut players: Query<&mut GridCoords, With<Player>>,
    input: Res<Input<KeyCode>>,
    level_walls: Res<LevelWalls>,
) {
    let movement_direction = if input.any_just_pressed([KeyCode::W, KeyCode::Up]) {
        GridCoords::new(0, 1)
    } else if input.any_just_pressed([KeyCode::A, KeyCode::Left]) {
        GridCoords::new(-1, 0)
    } else if input.any_just_pressed([KeyCode::S, KeyCode::Down]) {
        GridCoords::new(0, -1)
    } else if input.any_just_pressed([KeyCode::D, KeyCode::Right]) {
        GridCoords::new(1, 0)
    } else {
        return;
    };

    for mut player_grid_coords in players.iter_mut() {
        let destination = *player_grid_coords + movement_direction;
        if !level_walls.in_wall(&destination) {
            *player_grid_coords = destination;
        }
    }
}

pub fn translate_grid_coords_entities(
    mut grid_coords_entities: Query<(&mut Transform, &GridCoords), Changed<GridCoords>>,
) {
    for (mut transform, grid_coords) in grid_coords_entities.iter_mut() {
        transform.translation =
            grid_coords_to_translation(*grid_coords, IVec2::splat(GRID_SIZE))
                .extend(transform.translation.z);
    }
}

pub fn cache_wall_locations(
    mut level_walls: ResMut<LevelWalls>,
    mut level_events: EventReader<LevelEvent>,
    walls: Query<&GridCoords, With<Wall>>,
    ldtk_project_entities: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    for level_event in level_events.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            let ldtk_project = ldtk_project_assets
                .get(ldtk_project_entities.single())
                .expect("LdtkProject should be loaded when level is spawned");
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("spawned level should exist in project");
            let wall_locations = walls.iter().copied().collect();
            let new_level_walls = LevelWalls {
                wall_locations,
                level_width: level.px_wid / GRID_SIZE,
                level_height: level.px_hei / GRID_SIZE,
            };

            *level_walls = new_level_walls;
        }
    }
}

pub fn check_goal(
    level_selection: ResMut<LevelSelection>,
    players: Query<&GridCoords, (With<Player>, Changed<GridCoords>)>,
    goals: Query<&GridCoords, With<Goal>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if players
        .iter()
        .zip(goals.iter())
        .any(|(player_grid_coords, goal_grid_coords) | player_grid_coords == goal_grid_coords)
    {
        let indices = match level_selection.into_inner() {
            LevelSelection::Indices(indices) => indices,
            _ => panic!("level selection should always be Indices in this game"),
        };

        if indices.level < MAX_LEVEL_SELECTION - 1 {
            indices.level += 1;
        } else {
            app_state.set(AppState::GameOver);
        }
    }
}

pub fn check_pause(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_state.set(AppState::Pause);
    }
}

pub fn check_ldtk_transform(mut ldtk_projects: Query<&mut Transform, With<Handle<LdtkProject>>>) {
    let mut ldtk_projects_transform = ldtk_projects.single_mut();

    if ldtk_projects_transform.translation.x != 0.0 {
        println!("ldtk_projects_transform changed");
        ldtk_projects_transform.translation.x = 0.0;
    }
}
