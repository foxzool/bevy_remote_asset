# bevy_remote_asset

[![crates.io](https://img.shields.io/crates/v/bevy_remote_asset)](https://crates.io/crates/bevy_remote_asset)
[![crates.io](https://img.shields.io/crates/d/bevy_remote_asset)](https://crates.io/crates/bevy_cronjob)
[![Documentation](https://docs.rs/bevy_remote_asset/badge.svg)](https://docs.rs/bevy_remote_asset)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Seldom-SE/seldom_pixel#license)

 A Bevy plugin to load asset from web. It is based on the [ehttp](https://github.com/emilk/ehttp)

## Example

```rust
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

```