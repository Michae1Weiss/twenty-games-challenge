use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::state::GameState;

mod audio;
mod game;
mod state;
mod theme;
mod third_party;
mod ui;

pub fn plugin(app: &mut App) {
    app.add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::GameOver),
        )
        .add_plugins((game::plugin, audio::plugin));
}
