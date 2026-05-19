use bevy::app::App;
use bevy_asset_loader::prelude::*;

use crate::state::GameState;

mod audio;
mod game;
mod state;
mod theme;
mod third_party;
mod ui;

pub fn plugin(app: &mut App) {
    app.add_plugins(game::plugin).add_loading_state(
        // See: https://github.com/NiklasEi/bevy_asset_loader/blob/HEAD/bevy_asset_loader/examples/two_collections.rs
        // Also: https://github.com/NiklasEi/bevy_asset_loader/blob/74c74fabc4223eb734543739e9332c59c444f57f/bevy_asset_loader/examples/two_collections.rs#L15
        LoadingState::new(GameState::AssetLoading)
            .continue_to_state(GameState::GameOver)
            .load_collection::<game::assets::TextureAssets>()
            .load_collection::<audio::assets::AudioAssets>(),
    );
}
