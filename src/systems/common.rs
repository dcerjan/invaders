use bevy::prelude::*;

use crate::components::common::{ Player, Speed };


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
