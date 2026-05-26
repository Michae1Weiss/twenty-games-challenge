use bevy::prelude::*;

use crate::{
    game::{
        ball::{Ball, Spin},
        brick::Brick,
        collision::SpinEffect,
        paddle::Paddle,
    },
    state::GameState,
};

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(GameState = GameState::Playing)]
pub enum GamePhase {
    #[default]
    Open, // spin-ball: paddle imparts curve, no aim line
    Sniper, // few bricks left: slow-mo + straight aim line, spin disabled
}

/// The single source of truth for "when does the endgame start".
#[derive(Resource)]
pub struct GameRules {
    pub sniper_threshold: u32,
}

impl Default for GameRules {
    fn default() -> Self {
        Self {
            sniper_threshold: 5,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<GameRules>()
        .add_sub_state::<GamePhase>()
        .add_systems(
            Update,
            update_game_phase.run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnEnter(GamePhase::Sniper), enter_sniper);
}

fn update_game_phase(
    bricks: Query<(), With<Brick>>,
    rules: Res<GameRules>,
    phase: Res<State<GamePhase>>,
    mut next: ResMut<NextState<GamePhase>>,
) {
    let want = if bricks.iter().count() as u32 <= rules.sniper_threshold {
        GamePhase::Sniper
    } else {
        GamePhase::Open
    };
    if *phase.get() != want {
        next.set(want);
    }
}

/// Sniper is bullet-straight: clear any curve on the balls and stop the paddle
/// imparting new spin, so the predicted line matches reality exactly.
fn enter_sniper(
    mut balls: Query<&mut Spin, With<Ball>>,
    mut paddles: Query<&mut SpinEffect, With<Paddle>>,
) {
    for mut spin in &mut balls {
        spin.clear();
    }
    for mut effect in &mut paddles {
        *effect = SpinEffect::Impart(0.0);
    }
}
