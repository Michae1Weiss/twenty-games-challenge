use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(Resource, AssetCollection)]
pub struct UiAssets {
    #[asset(path = "textures/tape-01-transparent-320x64.png")]
    pub button: Handle<Image>,
    #[asset(path = "fonts/PermanentMarker-Regular.ttf")]
    pub font: Handle<Font>,
}
