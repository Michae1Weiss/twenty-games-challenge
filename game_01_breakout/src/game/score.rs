use bevy::prelude::*;

use crate::game::{Collision, EndRound, RestartRound, brick::BrickDestroyed, paddle::Paddle};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Score>()
        .add_systems(
            FixedUpdate,
            (
                update_score_on_brick_destruction,
                reset_multiplier_on_paddle_hit,
                reset_score,
            ),
        )
        .add_systems(Update, update_score_text);
}

const BRICK_POINTS: usize = 10;

#[derive(Resource)]
struct Score {
    score: usize,
    multiplier: usize,
}

impl Score {
    fn clear(&mut self) {
        self.score = 0;
        self.multiplier = 1;
    }
}

impl Default for Score {
    fn default() -> Self {
        Self {
            score: 0,
            multiplier: 1,
        }
    }
}

pub struct SpawnScoreCounter<M: Bundle> {
    marker: M,
}

impl<M: Bundle> SpawnScoreCounter<M> {
    pub fn new(marker: M) -> Self {
        Self { marker }
    }
}

#[derive(Component)]
struct ScoreText;

impl<M: Bundle> Command for SpawnScoreCounter<M> {
    fn apply(self, world: &mut World) -> () {
        let score = world.resource_scope(|_, score: Mut<Score>| score.score);

        world.spawn((
            ScoreText,
            Text2d::new(format!("Score: {}", score)),
            TextColor::BLACK,
            self.marker,
        ));
    }
}

fn update_score_on_brick_destruction(
    mut brick_destroyed_reader: MessageReader<BrickDestroyed>,
    mut score: ResMut<Score>,
) {
    for _ in brick_destroyed_reader.read() {
        let points = BRICK_POINTS * score.multiplier;
        score.score += points;
        info!(
            "+{} points | {} score | {} multiplier",
            points, score.score, score.multiplier
        );
        score.multiplier += 1;
    }
}

fn reset_multiplier_on_paddle_hit(
    mut collision_reader: MessageReader<Collision>,
    paddle: Query<(), With<Paddle>>,
    mut score: ResMut<Score>,
) {
    for collision in collision_reader.read() {
        if paddle.get(collision.hit).is_ok() {
            score.multiplier = 1;
        }
    }
}

fn reset_score(
    end_round_reader: MessageReader<EndRound>,
    restart_round_reader: MessageReader<RestartRound>,
    mut score: ResMut<Score>,
) {
    if !end_round_reader.is_empty() || !restart_round_reader.is_empty() {
        score.clear();
    }
}

fn update_score_text(score_texts: Query<&mut Text2d, With<ScoreText>>, score: Res<Score>) {
    for mut score_text in score_texts {
        score_text.0 = format!("Score: {:.0}", score.score);
    }
}
