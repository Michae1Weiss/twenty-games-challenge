use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

mod assets;
mod interaction;
mod main_menu;
mod pause_menu;
mod settings_menu;
mod stepper;
mod widget;

pub use assets::UiAssets;
pub use pause_menu::RestartGame;

use crate::{GameState, Screen};

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(GameState::AssetLoading).load_collection::<UiAssets>(),
    )
    .add_systems(OnEnter(GameState::Menu), enter_main_menu)
    .add_plugins((
        main_menu::plugin,
        pause_menu::plugin,
        interaction::plugin,
        settings_menu::plugin,
        stepper::plugin,
    ));
}

fn enter_main_menu(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Main);
}
