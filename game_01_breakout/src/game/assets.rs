use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(Resource, AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/paddle.png")]
    pub paddle: Handle<Image>,
    #[asset(path = "textures/brick.png")]
    pub brick: Handle<Image>,
}
