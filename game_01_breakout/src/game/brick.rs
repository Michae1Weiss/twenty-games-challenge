use bevy::prelude::*;

use crate::{
    GameState,
    game::{HalfSize, TextureAssets},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), spawn_bricks);
}

const BRICK_SIZE: Vec2 = Vec2::new(80., 40.);

#[derive(Component)]
pub struct Brick;

fn spawn_bricks(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    let n_rows: i32 = 6;
    let n_columns: i32 = 13;

    for row in 0..n_rows {
        for column in 0..n_columns {
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
