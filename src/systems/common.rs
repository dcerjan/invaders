use bevy::prelude::*;

use crate::components::{
  data::{ Speed, Life, Score },
  tags::{ Player, Projectile },
};


pub fn update_projectile(
  mut commands: Commands,
  time: Res<Time>,
  mut query: Query<(Entity, &mut Translation, &mut Life, &Speed, &Player, &Projectile)>,
) {
  for (id, mut translation, mut life, speed, _, _) in &mut query.iter() {
    *translation.0.y_mut() += time.delta_seconds * speed.0;
    life.0 -= time.delta_seconds;

    if life.0 < 0.0 {
      commands.despawn(id);
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
