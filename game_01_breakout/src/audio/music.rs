use bevy::{audio::Volume, prelude::*};

use crate::{
    audio::{AudioSettings, Music, assets::AudioAssets},
    state::GameState,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), play_game_music)
        .add_systems(OnEnter(GameState::Menu), play_menu_music)
        .add_systems(OnEnter(GameState::GameOver), play_game_over_music);
}

fn play_game_music(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    audio_settings: Res<AudioSettings>,
) {
    commands.spawn((
        AudioPlayer::new(audio_assets.game_music.clone()),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(audio_settings.music_volume.perceptual())),
        Music,
        DespawnOnExit(GameState::Playing),
    ));
}

fn play_menu_music(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    audio_settings: Res<AudioSettings>,
) {
    commands.spawn((
        AudioPlayer::new(audio_assets.menu_music.clone()),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(audio_settings.music_volume.perceptual())),
        Music,
        DespawnOnExit(GameState::Menu),
    ));
}

fn play_game_over_music(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    audio_settings: Res<AudioSettings>,
) {
    commands.spawn((
        AudioPlayer::new(audio_assets.game_over_music.clone()),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(audio_settings.music_volume.perceptual())),
        Music,
        DespawnOnExit(GameState::GameOver),
    ));
}
