use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(Resource, AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/paddle-oversimplified.png")]
    pub paddle: Handle<Image>,
    #[asset(path = "textures/brick-oversimplified.png")]
    pub brick: Handle<Image>,
    #[asset(path = "textures/background.png")]
    pub background: Handle<Image>,
    #[asset(path = "textures/danger-zone.png")]
    pub danger_zone: Handle<Image>,
}
