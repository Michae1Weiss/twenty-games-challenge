use bevy::prelude::*;

use crate::{
    CANVAS_SIZE,
    game::{
        ball::SpawnBall, brick::SpawnBricks, paddle::SpawnPaddle, respawn::SpawnRespawnArea, score::SpawnScoreCounter, wall::SpawnWalls
    },
};

pub const DEFAULT_PADDLE_SIZE: Vec2 = Vec2::new(200., 25.);

pub(super) fn plugin(app: &mut App) {
    app.add_message::<RestartRound>()
        .add_message::<StartRound>()
        .add_message::<EndRound>()
        .add_systems(
            Update,
            (despawn_round, spawn_round)
                .chain()
                .run_if(on_message::<RestartRound>),
        )
        .add_systems(Update, spawn_round.run_if(on_message::<StartRound>))
        .add_systems(Update, despawn_round.run_if(on_message::<EndRound>));
}

#[derive(Message, Default)]
pub struct RestartRound;

#[derive(Message, Default)]
pub struct EndRound;

#[derive(Message, Default)]
pub struct StartRound;

#[derive(Component, Clone, Copy)]
struct RoundEntity;

fn despawn_round(mut command: Commands, round_entities: Query<Entity, With<RoundEntity>>) {
    round_entities
        .iter()
        .for_each(|entity| command.entity(entity).despawn());
}

fn spawn_round(mut commands: Commands) {
    commands.queue(SpawnWalls::new(CANVAS_SIZE, RoundEntity));
    commands.queue(SpawnBricks::new(6, 13, RoundEntity));
    commands.queue(SpawnBall::new(
        Vec2::new(0.0, -50.0),
        Vec2::new(-20., -480.),
        RoundEntity,
    ));
    commands.queue(SpawnPaddle::new(DEFAULT_PADDLE_SIZE, RoundEntity));
    commands.queue(SpawnRespawnArea::new(RoundEntity));
    commands.queue(SpawnScoreCounter::new(RoundEntity));
}
