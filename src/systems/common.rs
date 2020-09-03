use bevy::prelude::*;

use crate::components::common::*;

pub fn player_movement(
  time: Res<Time>,
  keyboard_input: Res<Input<KeyCode>>,
  mut query: Query<(&Player, &mut Translation, &Speed)>,
) {
  for (_, mut translation, speed) in &mut query.iter() {
      let mut delta = 0.0;
      if keyboard_input.pressed(KeyCode::Left) { delta -= 1.0; }
      if keyboard_input.pressed(KeyCode::Right) { delta += 1.0; }

      *translation.0.x_mut() += time.delta_seconds * delta * speed.0;

      // Bound left and right positions
      *translation.0.x_mut() = f32::max(-380.0, f32::min(380.0, translation.0.x()));
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
      *bullet_translation.y_mut() += 48.0;
      *bullet_translation.z_mut() += 1.0;

      commands
        .spawn(SpriteComponents {
          draw: Draw { is_transparent: true, ..Default::default() },
          material: materials.add(preloaded_assets.player_projectile.into()),
          translation: bullet_translation,
          scale: Scale(4.0),
          sprite: Sprite {
            size: Vec2::new(8.0, 8.0),
          },
          ..Default::default()
        },)
        .with(Player())
        .with(Projectile())
        .with(Timer::from_seconds(1.0, false))
        .with(Damage(1));
    }
  }
}

pub fn update_projectile(
  mut commands: Commands,
  time: Res<Time>,
  mut query: Query<(Entity, &mut Translation, &Timer, &Player, &Projectile)>,
) {
  for (id, mut translation, timer, _, _) in &mut query.iter() {
    if timer.finished {
      commands.despawn(id);
    } else {
      *translation.0.y_mut() += time.delta_seconds * 600.0;
    }
  }
}

pub fn update_score(
  mut query: Query<(&Score, &mut Text)>,
) {
  for (score, mut text) in &mut query.iter() {
    text.value = format!("Score: {}", score.0);
  }
}
