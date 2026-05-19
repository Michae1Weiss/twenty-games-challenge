use std::default;

use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    AssetLoading,
    GameOver,
    Playing,
}

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum Screen {
    Main,
    Pause,
    Settings,
    #[default]
    None,
}
