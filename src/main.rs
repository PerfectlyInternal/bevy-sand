use bevy::prelude::*;

mod sand;
mod render;

fn main(){
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(sand::SandPlugin)
    .add_plugins(render::RenderPlugin)
    .run();
}
