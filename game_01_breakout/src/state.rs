use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<GameState>().init_state::<Screen>();
}

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    AssetLoading,
    GameOver,
    Playing,
    Menu,
}

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum Screen {
    Main,
    Pause,
    Settings,
    #[default]
    None,
}
