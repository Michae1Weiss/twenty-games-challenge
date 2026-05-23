use bevy::{input_focus::InputDispatchPlugin, prelude::*, ui_widgets::UiWidgetsPlugins};
use bevy_asset_loader::prelude::*;

mod audio;
mod camera;
mod game;
pub mod state;
mod theme;
mod third_party;
mod ui;

pub use camera::CANVAS_SIZE;
pub use state::{GameState, Screen};

pub fn plugin(app: &mut App) {
    app.add_plugins((DefaultPlugins, UiWidgetsPlugins, InputDispatchPlugin))
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Menu),
        )
        .add_plugins((
            camera::plugin,
            state::plugin,
            game::plugin,
            audio::plugin,
            ui::plugin,
        ));
}
