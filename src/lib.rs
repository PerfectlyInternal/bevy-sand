use bevy::prelude::*;
use bevy::window::PresentMode;
use wasm_bindgen::prelude::*;

mod render;
mod sand;
mod substances;
mod ui;

#[wasm_bindgen]
pub fn main() {
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
        .add_plugins(ui::UIPlugin)
        .run();
}
