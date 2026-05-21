use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

mod assets;
mod music;
mod settings;
pub mod sfx;
mod volume;

use assets::AudioAssets;
pub use settings::AudioSettings;
pub use sfx::PlaySfx;
pub use volume::Volume;

use crate::state::GameState;

#[derive(Component)]
struct Sfx;

#[derive(Component)]
struct Music;

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(GameState::AssetLoading).load_collection::<AudioAssets>(),
    )
    .add_plugins((sfx::plugin, music::plugin, settings::plugin));
}
