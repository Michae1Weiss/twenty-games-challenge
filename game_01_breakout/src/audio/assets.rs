use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "sfx/pop-02.ogg")]
    pub pop: Handle<AudioSource>,
    #[asset(path = "sfx/ball-hits-paddle.ogg")]
    pub ball_hits_paddle: Handle<AudioSource>,
    #[asset(path = "sfx/ball-hits-wall.ogg")]
    pub ball_hits_wall: Handle<AudioSource>,
}
