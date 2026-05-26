use bevy::prelude::*;

use crate::{
    GameState,
    game::{Brick, EndRound, brick::BrickDestroyed},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        check_victory
            .run_if(in_state(GameState::Playing))
            .after(super::ball::simulate_balls),
    );
}

fn check_victory(
    brick_destroyed_reader: MessageReader<BrickDestroyed>,
    bricks: Query<(), With<Brick>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut end_round: MessageWriter<EndRound>,
) {
    let brick_was_removed = !brick_destroyed_reader.is_empty();
    if brick_was_removed && bricks.is_empty() {
        end_round.write(EndRound);
        game_state.set(GameState::Won);
    }
}
