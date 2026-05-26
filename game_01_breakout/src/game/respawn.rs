use bevy::{
    color::palettes::{css::RED, tailwind::RED_200},
    prelude::*,
    sprite::Anchor,
};

use crate::{
    CANVAS_SIZE,
    game::{TextureAssets, round::DEFAULT_PADDLE_SIZE},
};

#[derive(Component)]
pub struct RespawnBallArea;

pub struct SpawnRespawnArea<M: Bundle> {
    marker: M,
}

impl<M: Bundle> SpawnRespawnArea<M> {
    pub fn new(marker: M) -> Self {
        Self { marker }
    }
}

impl<M: Bundle> Command for SpawnRespawnArea<M> {
    fn apply(self, world: &mut World) -> () {
        let texture = world.resource_scope(|_, texture_assets: Mut<TextureAssets>| {
            texture_assets.danger_zone.clone()
        });

        // TODO: remove constant coupling
        world.spawn((
            Sprite {
                color: Color::from(RED).with_alpha(0.2),
                custom_size: Some(Vec2::new(
                    CANVAS_SIZE.x,
                    CANVAS_SIZE.y / 8.0 - DEFAULT_PADDLE_SIZE.y / 2.0 - 60.0,
                )),
                ..default()
            },
            Anchor::BOTTOM_CENTER,
            Transform::from_xyz(0.0, -CANVAS_SIZE.y / 2. + 60.0, -1.0),
            RespawnBallArea,
            self.marker,
        ));
    }
}
