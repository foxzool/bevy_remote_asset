use bevy::app::{App, Plugin};
use bevy::asset::{AssetPlugin, AssetServer};

use crate::remote_asset_io::RemoterAssetIo;

#[derive(Default)]
pub struct RemoteAssetPlugin;

impl Plugin for RemoteAssetPlugin {
    fn build(&self, app: &mut App) {
        let asset_io = RemoterAssetIo {
            default_io: AssetPlugin::default().create_platform_default_asset_io(),
        };

        app.insert_resource(AssetServer::new(asset_io));
    }
}
