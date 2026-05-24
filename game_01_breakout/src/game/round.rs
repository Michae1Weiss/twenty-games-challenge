use bevy::prelude::*;

use crate::game::{TextureAssets, ball::SpawnBall, brick::spawn_grid_of_bricks};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (despawn_round, spawn_round)
            .chain()
            .run_if(on_message::<RestartRound>),
    )
    .add_systems(Update, spawn_round.run_if(on_message::<StartRound>));
}

#[derive(Message)]
pub struct RestartRound;

#[derive(Message)]
pub struct StartRound;

#[derive(Component, Clone)]
struct RoundEntity;

fn despawn_round(mut command: Commands, round_entities: Query<Entity, With<RoundEntity>>) {
    round_entities
        .iter()
        .for_each(|entity| command.entity(entity).despawn());
}

fn spawn_round(mut commands: Commands, textures: Res<TextureAssets>) {
    spawn_grid_of_bricks(&mut commands, &textures, RoundEntity);
    commands.queue(SpawnBall::new(
        Vec2::new(0.0, -50.0),
        Vec2::new(-20., -480.),
        RoundEntity,
    ));
}
