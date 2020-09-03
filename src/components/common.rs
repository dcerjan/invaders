use bevy::prelude::*;

#[derive(Debug)]
pub struct Player();

#[derive(Debug)]
pub struct Ship();

#[derive(Debug)]
pub struct Projectile();

#[derive(Debug)]
pub struct Enemy();

#[derive(Debug)]
pub struct Score(pub u32);

#[derive(Debug)]
pub struct Health(pub u8);

#[derive(Debug)]
pub struct Lives(pub u8);

#[derive(Debug)]
pub struct Damage(pub u8);

#[derive(Debug)]
pub struct Speed(pub f32);


#[derive(Debug, Default)]
pub struct PreloadedAssets {
  pub player_ship: Handle<Texture>,
  pub player_projectile: Handle<Texture>,

  pub default_font: Handle<Font>,
}
