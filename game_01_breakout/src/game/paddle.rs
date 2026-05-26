use bevy::prelude::*;
use bevy_transform_interpolation::prelude::TransformInterpolation;

use crate::{
    CANVAS_SIZE,
    audio::{CollisionSfx, PlaySfx},
    game::{
        HalfSize, PaddleMovement, TextureAssets,
        collision::{Collider, CollisionResponse, SpinEffect}, phase::GamePhase,
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, impart_spin.run_if(in_state(GamePhase::Open)));
}

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
            Collider::Aabb {
                half_size: self.size / 2.,
            },
            CollisionResponse::Deflect,
            CollisionSfx(PlaySfx::BallPaddle),
            SpinEffect::Impart(0.0),
            self.marker,
        ));
    }
}

fn impart_spin(mut paddles: Query<(&PaddleMovement, &mut SpinEffect), With<Paddle>>) {
    for (movement, mut effect) in &mut paddles {
        *effect = SpinEffect::Impart(match movement {
            PaddleMovement::Left => -0.005, // TODO: move to contants
            PaddleMovement::Right => 0.005, // TODO: move to contants
            PaddleMovement::Idle => 0.0,
        });
    }
}
