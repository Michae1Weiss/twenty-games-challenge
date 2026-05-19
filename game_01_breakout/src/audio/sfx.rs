use bevy::{audio::Volume, prelude::*};

use crate::state::GameState;

use super::{Sfx, assets::AudioAssets};

#[derive(Message)]
pub enum PlaySfx {
    BallPaddle,
    BallWall,
    BrickBreak,
}

pub(super) fn plugin(app: &mut App) {
    app.add_message::<PlaySfx>().add_systems(
        Update,
        play_sfx.run_if(not(in_state(GameState::AssetLoading))),
    );
}

fn play_sfx(
    mut play_sfx_reader: MessageReader<PlaySfx>,
    audio_assets: Res<AudioAssets>,
    mut commands: Commands,
) {
    for play_sfx in play_sfx_reader.read() {
        let handle = match play_sfx {
            PlaySfx::BallPaddle => &audio_assets.ball_hits_paddle,
            PlaySfx::BallWall => &audio_assets.ball_hits_wall,
            PlaySfx::BrickBreak => &audio_assets.pop,
        };

        commands.spawn((
            AudioPlayer::new(handle.clone()),
            PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.5)),
            Sfx,
        ));
    }
}
