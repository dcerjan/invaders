use bevy::{
  prelude::*,
  window::WindowMode,
  render::pass::ClearColor,
};


pub struct WindowSetupPlugin;
impl Plugin for WindowSetupPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_resource(WindowDescriptor {
        title: "Invaders!".to_string(),
        width: 320,
        height: 200,
        vsync: true,
        resizable: false,
        mode: WindowMode::Windowed,
        ..Default::default()
      })
      .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
      ;
  }
}
