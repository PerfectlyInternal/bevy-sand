use bevy::prelude::*;
use bevy::window::PresentMode;

mod sand;
mod substances;
mod render;

fn main(){
  App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "bevy_sand".into(),
            // disable vsync to get a better idea of performance
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        }),
        ..default()
    }))
    .add_plugins(sand::SandPlugin)
    .add_plugins(render::RenderPlugin)
    .run();
}
