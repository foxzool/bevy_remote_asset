use bevy::prelude::*;

use bevy_remote_asset::RemoteAssetPlugin;

fn main() {
    App::new()
        .add_plugins(RemoteAssetPlugin)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: asset_server
            .load("https://seeklogo.com/images/B/bevy-engine-logo-25F6DD58BF-seeklogo.com.png"),
        ..default()
    });
}
