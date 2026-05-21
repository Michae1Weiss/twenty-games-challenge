// menu/settings_menu.rs
use crate::{
    GameState, Screen,
    audio::{AudioSettings, Volume},
    ui::{stepper::Stepper, stepper::stepper, widget::*},
};
use bevy::prelude::*;

#[derive(Component)]
struct MusicVolume;
#[derive(Component)]
struct SfxVolume;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Settings), spawn)
        .add_systems(
            Update,
            (bind_music, bind_sfx).run_if(in_state(Screen::Settings)),
        );
}

fn spawn(mut commands: Commands, settings: Res<AudioSettings>) {
    commands.spawn((
        ui_root("SettingsMenu"),
        DespawnOnExit(Screen::Settings),
        children![
            header("Settings"),
            stepper(
                "Music",
                settings.music.as_steps(),
                Volume::STEPS,
                MusicVolume
            ),
            stepper("Sound", settings.sfx.as_steps(), Volume::STEPS, SfxVolume),
            button("Back", back),
        ],
    ));
}

fn bind_music(
    q: Query<&Stepper, (Changed<Stepper>, With<MusicVolume>)>,
    mut settings: ResMut<AudioSettings>,
) {
    for stepper in &q {
        settings.music = Volume::from_steps(stepper.value);
    }
}

fn bind_sfx(
    q: Query<&Stepper, (Changed<Stepper>, With<SfxVolume>)>,
    mut settings: ResMut<AudioSettings>,
) {
    for stepper in &q {
        settings.sfx = Volume::from_steps(stepper.value);
    }
}

fn back(
    _: On<Pointer<Click>>,
    game_state: Res<State<GameState>>,
    mut next: ResMut<NextState<Screen>>,
) {
    next.set(match game_state.get() {
        GameState::Menu => Screen::Main,
        GameState::Playing => Screen::Pause,
        GameState::AssetLoading | GameState::GameOver => unreachable!(),
    });
}
