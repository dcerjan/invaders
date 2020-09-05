use bevy::prelude::*;


#[derive(Debug, Default)]
pub struct PreloadedAssets {
  pub player_ship: Handle<Texture>,
  pub player_projectile: Handle<Texture>,

  pub enemy_ship_1: Handle<Texture>,

  pub default_font: Handle<Font>,
}
