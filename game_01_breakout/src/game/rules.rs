use bevy::prelude::*;

use crate::{GameState, game::Brick};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        check_victory
            .run_if(in_state(GameState::Playing))
            .after(super::ball::ball_movement),
    );
}

fn check_victory(
    mut removed_bricks: RemovedComponents<Brick>,
    bricks: Query<(), With<Brick>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let brick_was_removed = !removed_bricks.is_empty();
    removed_bricks.clear();

    if brick_was_removed && bricks.is_empty() {
        game_state.set(GameState::Won);
    }
}
