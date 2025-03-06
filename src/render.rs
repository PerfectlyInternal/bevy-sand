use bevy::prelude::*;
use bevy::render::{
    render_asset::RenderAssetUsages,
    render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy::color::palettes::css;

use crate::sand::Universe;

pub struct RenderPlugin;
impl bevy::app::Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(FixedUpdate, draw);
    }
}

#[derive(Resource)]
struct OutputImage(Handle<Image>);

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2d);

    // create an image that we are going to draw into
    let image = Image::new_fill(
        // 2D image of size 256x256
        Extent3d {
            width: 256,
            height: 256,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &(css::RED.to_u8_array()),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    let handle = images.add(image);
    
    commands.spawn(Sprite::from_image(handle.clone()));
    commands.insert_resource(OutputImage(handle));
}

fn draw(handle: Res<OutputImage>, mut images: ResMut<Assets<Image>>, universe: Res<Universe>) {
    let image = images.get_mut(&handle.0).expect("Image not found");
    for x in 0..universe.grid.cols() {
        for y in 0..universe.grid.rows() {
            let _ = image.set_color_at(
                x.try_into().unwrap(),
                y.try_into().unwrap(),
                universe.grid.get(x, y).unwrap().color);
        }
    }
}
