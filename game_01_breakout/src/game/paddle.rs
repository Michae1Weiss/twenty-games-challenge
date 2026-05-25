use bevy::prelude::*;
use bevy_transform_interpolation::prelude::TransformInterpolation;

use crate::{
    CANVAS_SIZE,
    game::{HalfSize, PaddleMovement, TextureAssets},
};

#[derive(Component)]
pub struct Paddle;

pub struct SpawnPaddle<M: Bundle> {
    size: Vec2,
    marker: M,
}

impl<M: Bundle> SpawnPaddle<M> {
    pub fn new(size: Vec2, marker: M) -> Self {
        Self { size, marker }
    }
}

impl<M: Bundle> Command for SpawnPaddle<M> {
    fn apply(self, world: &mut World) -> () {
        let paddle_image = world
            .resource_scope(|_, texture_assets: Mut<TextureAssets>| texture_assets.paddle.clone());

        world.spawn((
            Sprite {
                image: paddle_image,
                custom_size: Some(self.size),
                ..default()
            },
            Transform::from_xyz(0.0, -CANVAS_SIZE.y * 3.0 / 8.0, 0.0),
            TransformInterpolation,
            Paddle,
            PaddleMovement::default(),
            HalfSize(self.size / 2.),
            self.marker,
        ));
    }
}
