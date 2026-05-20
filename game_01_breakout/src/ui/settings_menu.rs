use bevy::{prelude::*, ui_widgets::ValueChange};

use crate::{GameState, Screen, audio::AudioSettings, ui::widget::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Settings), spawn);
}

fn spawn(mut commands: Commands, settings: Res<AudioSettings>) {
    commands.spawn((
        ui_root("SettingsMenu"),
        DespawnOnExit(Screen::Settings),
        children![
            header("Settings"),
            slider_row("Music", settings.music_volume, on_music_change),
            slider_row("Sound", settings.sfx_volume, on_sfx_change),
            button("Back", back),
        ],
    ));
}

fn on_music_change(change: On<ValueChange<f32>>, mut s: ResMut<AudioSettings>) {
    s.music_volume = change.value;
}

fn on_sfx_change(change: On<ValueChange<f32>>, mut s: ResMut<AudioSettings>) {
    s.sfx_volume = change.value;
}

/// Back returns to wherever we came from: Main if GameState::Menu, Pause if GameState::InGame.
fn back(
    _: On<Pointer<Click>>,
    game_state: Res<State<GameState>>,
    mut next: ResMut<NextState<Screen>>,
) {
    next.set(match game_state.get() {
        GameState::Menu => Screen::Main,
        GameState::Playing => Screen::Pause,
        GameState::AssetLoading => unreachable!(),
        GameState::GameOver => unreachable!(),
    });
}
