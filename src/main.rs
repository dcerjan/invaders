use bevy::prelude::*;

use invaders::components::{
  preloaded_assets::*,
  data::*,
};
use invaders::plugins::*;
use invaders::systems::{
  preload_assets,
  player::*,
  common::*,
};

fn setup(
  mut commands: Commands,
  preloaded_assets: Res<PreloadedAssets>,
) {
  commands
    .spawn(Camera2dComponents::default())
    .spawn(UiCameraComponents::default())
    // TODO: Split ui stuff
    .spawn(TextComponents {
      style: Style {
          align_self: AlignSelf::FlexEnd,
          margin: Rect::all(Val::Px(4.0)),
          ..Default::default()
      },
      text: Text {
          value: "Score: 0".to_string(),
          font: preloaded_assets.default_font,
          style: TextStyle {
              font_size: 20.0,
              color: Color::WHITE,
          },
      },
      ..Default::default()
    })
    .with(Score(10));
}

fn main() {
  App::build()
    .add_plugin(WindowSetupPlugin)
    .add_default_plugins()
    .add_resource(PreloadedAssets::default())
    .add_startup_system(preload_assets.system())
    .add_startup_system(setup.system())
    .add_startup_system(player_init.system())
    .add_system(player_movement.system())
    .add_system(player_shoot.system())
    .add_system(update_projectile.system())
    .add_system(update_score.system())
    .run();
}
