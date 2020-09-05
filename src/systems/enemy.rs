use bevy::prelude::*;


use crate::components::{
  preloaded_assets::PreloadedAssets,
  data::{ Speed, Health },
  tags::{ Ship, Enemy },
};


pub fn enemy_init(
  mut commands: Commands,
  preloaded_assets: Res<PreloadedAssets>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  let enemy_texture = preloaded_assets.enemy_ship_1;

  commands
    .spawn(
      SpriteComponents {
        draw: Draw { is_transparent: true, ..Default::default() },
        material: materials.add(enemy_texture.into()),
        translation: Translation(Vec3::new(0.0, 68.0, 0.0)),
        scale: Scale(1.0),
        sprite: Sprite {
          size: Vec2::new(32.0, 32.0),
        },
        ..Default::default()
      },
    )
    .with(Enemy())
    .with(Ship())
    .with(Speed(144.0))
    .with(Health(1))
    ;
}

pub fn enemy_movement(
  time: Res<Time>,
  mut query: Query<(&Enemy, &mut Translation, &Speed, &Ship)>,
) {
  for (_, mut translation, speed, _) in &mut query.iter() {
      let pos_x = f32::sin((time.seconds_since_startup as f32) * 0.5);

      translation.set_x(pos_x * speed.0);
  }
}
