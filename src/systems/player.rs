use bevy::prelude::*;

use crate::components::{
  preloaded_assets::PreloadedAssets,
  data::{ Speed, Life, Damage, Health, Lives },
  tags::{ Player, Ship, Projectile },
};


pub fn player_init(
  mut commands: Commands,
  preloaded_assets: Res<PreloadedAssets>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  let spaceship_texture = preloaded_assets.player_ship;

  commands
    .spawn(
      SpriteComponents {
        draw: Draw { is_transparent: true, ..Default::default() },
        material: materials.add(spaceship_texture.into()),
        translation: Translation(Vec3::new(0.0, -68.0, 0.0)),
        scale: Scale(1.0),
        sprite: Sprite {
          size: Vec2::new(32.0, 32.0),
        },
        ..Default::default()
      },
    )
    .with(Player())
    .with(Ship())
    .with(Lives(3))
    .with(Speed(128.0))
    .with(Health(3))
    ;
}

pub fn player_movement(
  time: Res<Time>,
  keyboard_input: Res<Input<KeyCode>>,
  mut query: Query<(&Player, &mut Translation, &Speed, &Ship)>,
) {
  for (_, mut translation, speed, _) in &mut query.iter() {
      let mut delta = 0.0;
      if keyboard_input.pressed(KeyCode::Left) { delta -= 1.0; }
      if keyboard_input.pressed(KeyCode::Right) { delta += 1.0; }

      *translation.0.x_mut() += time.delta_seconds * delta * speed.0;
      *translation.0.x_mut() = f32::max(-144.0, f32::min(144.0, translation.0.x()));
  }
}

pub fn player_shoot(
  mut commands: Commands,
  keyboard_input: Res<Input<KeyCode>>,
  preloaded_assets: Res<PreloadedAssets>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut query: Query<(&Player, &Translation, &Ship)>,
) {
  for (_, translation, _) in &mut query.iter() {
    if keyboard_input.just_pressed(KeyCode::Space) {
      let mut bullet_translation = translation.clone();
      *bullet_translation.y_mut() += 16.0;

      commands
        .spawn(SpriteComponents {
          draw: Draw { is_transparent: true, ..Default::default() },
          material: materials.add(preloaded_assets.player_projectile.into()),
          translation: bullet_translation,
          scale: Scale(1.0),
          sprite: Sprite {
            size: Vec2::new(8.0, 8.0),
          },
          ..Default::default()
        },)
        .with(Player())
        .with(Projectile())
        .with(Life(1.0))
        .with(Speed(128.0))
        .with(Damage(1));
    }
  }
}
