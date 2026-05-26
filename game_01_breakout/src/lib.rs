use bevy::{input_focus::InputDispatchPlugin, prelude::*, window::WindowMode};
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
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // Use Fullscreen on the primary monitor
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }),
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
