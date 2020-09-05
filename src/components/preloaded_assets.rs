use bevy::prelude::*;


#[derive(Debug, Default)]
pub struct PreloadedAssets {
  pub player_ship: Handle<Texture>,
  pub player_projectile: Handle<Texture>,

  pub default_font: Handle<Font>,
}
