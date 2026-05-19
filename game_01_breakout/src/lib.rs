use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

mod audio;
mod game;
pub mod state;
mod theme;
mod third_party;
mod ui;

pub use state::{GameState, Screen};

pub fn plugin(app: &mut App) {
    app.add_plugins(DefaultPlugins)
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::GameOver),
        )
        .add_plugins((state::plugin, game::plugin, audio::plugin, ui::plugin));
}
