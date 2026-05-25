use bevy::{input_focus::InputDispatchPlugin, prelude::*};
use bevy_asset_loader::prelude::*;

mod audio;
mod camera;
mod crt;
mod game;
pub mod state;
mod theme;
mod third_party;
mod ui;

use bevy_transform_interpolation::prelude::TransformInterpolationPlugin;
pub use camera::CANVAS_SIZE;
pub use state::{GameState, Screen};
pub use ui::RestartGame;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        DefaultPlugins,
        InputDispatchPlugin,
        TransformInterpolationPlugin::default(),
    ))
    .add_loading_state(
        LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Menu),
    )
    .add_plugins((
        camera::plugin,
        state::plugin,
        game::plugin,
        audio::plugin,
        ui::plugin,
        crt::plugin,
    ));
}
