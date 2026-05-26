use bevy::prelude::*;

use crate::{
    game::StartRound,
    state::{GameState, Screen},
    ui::{UiAssets, widget::*},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Main), spawn);
}

fn spawn(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands.spawn((
        ui_root("MainMenu", ui_assets.background_menu.clone()),
        DespawnOnExit(Screen::Main),
        children![
            header("Breakout", ui_assets.font.clone()),
            text("A / D — Move paddle", ui_assets.font.clone()),
            button(
                "Play",
                ui_assets.button.clone(),
                ui_assets.font.clone(),
                enter_game
            ),
            button(
                "Settings",
                ui_assets.button.clone(),
                ui_assets.font.clone(),
                open_settings
            ),
            button(
                "Quit",
                ui_assets.button.clone(),
                ui_assets.font.clone(),
                quit_app
            ),
        ],
    ));
}

fn enter_game(
    _: On<Pointer<Click>>,
    mut next_app: ResMut<NextState<GameState>>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut start_round: MessageWriter<StartRound>,
) {
    next_app.set(GameState::Playing);
    next_screen.set(Screen::None);
    start_round.write(StartRound);
}

fn open_settings(_: On<Pointer<Click>>, mut next: ResMut<NextState<Screen>>) {
    next.set(Screen::Settings);
}

fn quit_app(_: On<Pointer<Click>>, mut exit: MessageWriter<AppExit>) {
    exit.write(AppExit::Success);
}
