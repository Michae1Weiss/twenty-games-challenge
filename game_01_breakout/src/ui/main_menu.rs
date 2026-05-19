use bevy::prelude::*;

use crate::{
    state::{GameState, Screen},
    ui::widget::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Main), spawn);
}

fn spawn(mut commands: Commands) {
    commands.spawn((
        ui_root("MainMenu"),
        DespawnOnExit(Screen::Main),
        children![
            header("Breakout"),
            button("Play", enter_game),
            button("Settings", open_settings),
            button("Quit", quit_app),
        ],
    ));
}

fn enter_game(
    _: On<Pointer<Click>>,
    mut next_app: ResMut<NextState<GameState>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    next_app.set(GameState::Playing);
    next_screen.set(Screen::None);
}

fn open_settings(_: On<Pointer<Click>>, mut next: ResMut<NextState<Screen>>) {
    next.set(Screen::Settings);
}

fn quit_app(_: On<Pointer<Click>>, mut exit: MessageWriter<AppExit>) {
    exit.write(AppExit::Success);
}
