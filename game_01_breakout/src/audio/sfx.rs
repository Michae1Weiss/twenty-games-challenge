use bevy::{audio::Volume, prelude::*};

use crate::{audio::AudioSettings, game::Collision, state::GameState};

use super::{Sfx, assets::AudioAssets};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<PlaySfx>().add_systems(
        Update,
        (collision_sfx, play_sfx).run_if(not(in_state(GameState::AssetLoading))),
    );
}

#[derive(Message, Clone, Copy)]
pub enum PlaySfx {
    BallPaddle,
    BallWall,
    BrickBreak,
}

#[derive(Component, Clone, Copy)]
pub struct CollisionSfx(pub PlaySfx);

fn play_sfx(
    mut play_sfx_reader: MessageReader<PlaySfx>,
    audio_assets: Res<AudioAssets>,
    audio_settings: Res<AudioSettings>,
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
            PlaybackSettings::DESPAWN
                .with_volume(Volume::Linear(audio_settings.sfx_volume.perceptual())),
            Sfx,
        ));
    }
}

fn collision_sfx(
    mut collisions: MessageReader<Collision>,
    sounds: Query<&CollisionSfx>,
    mut play_sfx_writer: MessageWriter<PlaySfx>,
) {
    for collision in collisions.read() {
        if let Ok(&CollisionSfx(sfx)) = sounds.get(collision.hit) {
            play_sfx_writer.write(sfx);
        }
    }
}
