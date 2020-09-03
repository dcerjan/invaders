use bevy::{
  prelude::*,
  render::pass::ClearColor,
};

use invaders::components::common::*;
use invaders::systems::{ preload_assets, common::* } ;

fn setup(
  mut commands: Commands,
  preloaded_assets: Res<PreloadedAssets>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  let spaceship_texture = preloaded_assets.player_ship;

  commands
    .spawn(Camera2dComponents::default())
    .spawn(UiCameraComponents::default())
    .spawn(
      SpriteComponents {
        draw: Draw { is_transparent: true, ..Default::default() },
        material: materials.add(ColorMaterial::modulated_texture(spaceship_texture, Color::rgba(1.0, 1.0, 1.0, 1.0))),
        translation: Translation(Vec3::new(0.0, -256.0, 0.0)),
        scale: Scale(4.0),
        sprite: Sprite {
          size: Vec2::new(32.0, 32.0),
        },
        ..Default::default()
      },
    )
    .with(Player())
    .with(Ship())
    .with(Lives(3))
    .with(Speed(400.0))
    .with(Health(3))
    // TODO: Split ui stuff
    .spawn(TextComponents {
      style: Style {
          align_self: AlignSelf::FlexEnd,
          margin: Rect::all(Val::Px(20.0)),
          ..Default::default()
      },
      text: Text {
          value: "Score: 0".to_string(),
          font: preloaded_assets.default_font,
          style: TextStyle {
              font_size: 60.0,
              color: Color::WHITE,
          },
      },
      ..Default::default()
    })
    .with(Score(10));
}

fn main() {
  App::build()
    .add_default_plugins()
    .add_resource(ClearColor(Color::rgb(0.4, 0.6, 0.7)))
    .add_resource(PreloadedAssets::default())
    .add_startup_system(preload_assets.system())
    .add_startup_system(setup.system())
    .add_system(player_movement.system())
    .add_system(player_shoot.system())
    .add_system(update_projectile.system())
    .add_system(update_score.system())
    .run();
}
