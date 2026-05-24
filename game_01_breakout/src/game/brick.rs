use bevy::prelude::*;

use crate::{
    GameState,
    game::{HalfSize, TextureAssets},
};

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(OnEnter(GameState::Playing), spawn_bricks);
}

const BRICK_SIZE: Vec2 = Vec2::new(80., 40.);
const N_BRICK_ROWS: u32 = 1;
const N_BRICK_COLUMNS: u32 = 2;

#[derive(Component)]
pub struct Brick;

/// Constructor for a single brick at a grid cell
pub fn brick(row: u32, column: u32, texture: Handle<Image>) -> impl Bundle {
    (
        Brick,
        Sprite {
            image: texture,
            custom_size: Some(BRICK_SIZE),
            ..default()
        },
        Transform::from_xyz(
            -480.0 + BRICK_SIZE.x * column as f32,
            240.0 - BRICK_SIZE.y * row as f32,
            0.0,
        ),
        HalfSize(BRICK_SIZE / 2.0),
        Name::new("Brick"),
    )
}

pub fn spawn_grid_of_bricks(
    commands: &mut Commands,
    textures: &TextureAssets,
    marker: impl Bundle + Clone,
) {
    for row in 0..N_BRICK_ROWS {
        for column in 0..N_BRICK_COLUMNS {
            commands.spawn((brick(row, column, textures.brick.clone()), marker.clone()));
        }
    }
}

fn spawn_bricks(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    for row in 0..N_BRICK_ROWS {
        for column in 0..N_BRICK_COLUMNS {
            commands.spawn((
                Brick,
                Sprite {
                    image: texture_assets.brick.clone(),
                    custom_size: Some(BRICK_SIZE),
                    ..default()
                },
                Transform::from_xyz(
                    -480. + BRICK_SIZE.x * column as f32,
                    240. - BRICK_SIZE.y * row as f32,
                    0.0,
                ),
                HalfSize(BRICK_SIZE / 2.),
                DespawnOnExit(GameState::Playing),
            ));
        }
    }
}
