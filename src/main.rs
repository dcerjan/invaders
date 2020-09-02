use bevy::{
  prelude::*,
  render::pass::ClearColor,
};

use invaders::components::common::*;
use invaders::systems::common::*;

fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  let spaceship_texture = asset_server.load("assets/sprite/player_ship.png").unwrap();

  commands
    .spawn(Camera2dComponents::default())
    .spawn(UiCameraComponents::default())
    .spawn(
      SpriteComponents {
        material: materials.add(spaceship_texture.into()),
        translation: Translation(Vec3::new(0.0, -256.0, 0.0)),
        scale: Scale(4.0),
        sprite: Sprite {
          size: Vec2::new(32.0, 32.0),
        },
        ..Default::default()
      },
    )
    .with(Player())
    .with(Score(0))
    .with(Lives(3))
    .with(Speed(400.0))
    .with(Health(3));
}


fn main() {
  App::build()
    .add_default_plugins()
    .add_resource(ClearColor(Color::rgb(0.4, 0.6, 0.7)))
    .add_startup_system(setup.system())
    .add_system(player_movement.system())
    .run();
}
