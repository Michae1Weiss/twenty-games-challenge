use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    // Music
    #[asset(path = "music/game-music.ogg")]
    pub game_music: Handle<AudioSource>,
    #[asset(path = "music/menu-music.ogg")]
    pub menu_music: Handle<AudioSource>,
    #[asset(path = "music/game-over-music.ogg")]
    pub game_over_music: Handle<AudioSource>,
    // SFX
    #[asset(path = "sfx/pop-02.ogg")]
    pub pop: Handle<AudioSource>,
    #[asset(path = "sfx/ball-hits-paddle.ogg")]
    pub ball_hits_paddle: Handle<AudioSource>,
    #[asset(path = "sfx/ball-hits-wall.ogg")]
    pub ball_hits_wall: Handle<AudioSource>,
    #[asset(path = "sfx/applause.ogg")]
    pub applause: Handle<AudioSource>,
    #[asset(path = "sfx/button_hover_10x_lauder.ogg")]
    pub button_hover: Handle<AudioSource>,
    #[asset(path = "sfx/button_press_10x_lauder.ogg")]
    pub button_press: Handle<AudioSource>,
}
