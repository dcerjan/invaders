use bevy::prelude::*;

use crate::components::preloaded_assets::PreloadedAssets;


pub fn preload_assets(
  asset_server: Res<AssetServer>,
  mut preloaded_assets: ResMut<PreloadedAssets>
) {
  preloaded_assets.default_font = asset_server.load::<Font, &str>("assets/fonts/AmaticSC-Bold.ttf").unwrap();

  preloaded_assets.player_projectile = asset_server.load("assets/sprite/player_projectile.png").unwrap();
  preloaded_assets.player_ship = asset_server.load("assets/sprite/player_ship.png").unwrap();

  asset_server
    .watch_for_changes()
    .unwrap();
}
