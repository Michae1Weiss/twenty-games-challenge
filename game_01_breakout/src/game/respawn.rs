use bevy::{prelude::*, sprite::Anchor};

use crate::{
    CANVAS_SIZE,
    game::{DEFAULT_PADDLE_SIZE, TextureAssets},
};

#[derive(Component)]
pub struct RespawnBallArea;

struct SpawnRespawnArea<M: Bundle> {
    marker: M,
}

impl<M: Bundle> Command for SpawnRespawnArea<M> {
    fn apply(self, world: &mut World) -> () {
        let texture = world.resource_scope(|_, texture_assets: Mut<TextureAssets>| {
            texture_assets.danger_zone.clone()
        });

        // TODO: remove constant coupling
        world.spawn((
            Sprite {
                image: texture,
                custom_size: Some(Vec2::new(
                    CANVAS_SIZE.x,
                    CANVAS_SIZE.y / 8.0 - DEFAULT_PADDLE_SIZE.y / 2.0,
                )),
                ..default()
            },
            Anchor::BOTTOM_CENTER,
            Transform::from_xyz(0.0, -CANVAS_SIZE.y / 2., -1.0),
            RespawnBallArea,
            self.marker,
        ));
    }
}
