use bevy::prelude::*;

use crate::{GameState, Screen, game::RestartRound, ui::widget::*};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<RestartGame>()
        .add_systems(OnEnter(Screen::Pause), spawn)
        .add_systems(Update, toggle_pause.run_if(in_state(GameState::Playing)));
}

#[derive(Message)]
pub struct RestartGame;

fn spawn(mut commands: Commands) {
    commands.spawn((
        ui_root("PauseMenu"),
        DespawnOnExit(Screen::Pause),
        children![
            header("Paused"),
            button("Continue", resume),
            button("Restart", restart),
            button("Settings", open_settings),
            button("Main Menu", to_main_menu),
        ],
    ));
}

fn resume(_: On<Pointer<Click>>, mut next: ResMut<NextState<Screen>>) {
    next.set(Screen::None);
}

fn restart(
    _: On<Pointer<Click>>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut restart_round: MessageWriter<RestartRound>,
) {
    restart_round.write(RestartRound);
    next_screen.set(Screen::None);
}

fn open_settings(_: On<Pointer<Click>>, mut next: ResMut<NextState<Screen>>) {
    next.set(Screen::Settings);
}

fn to_main_menu(
    _: On<Pointer<Click>>,
    mut next_app: ResMut<NextState<GameState>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    next_app.set(GameState::Menu);
    next_screen.set(Screen::Main);
}

fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    current: Res<State<Screen>>,
    mut next: ResMut<NextState<Screen>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match current.get() {
            Screen::None => next.set(Screen::Pause),
            Screen::Pause => next.set(Screen::None),
            _ => {} // Settings open: handled by Settings' back button
        }
    }
}
