use bevy::prelude::*;
use bevy::render::{
    render_asset::RenderAssetUsages,
    render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy::color::palettes::css;
use iyes_perf_ui::prelude::*;
use iyes_perf_ui::entries::PerfUiFramerateEntries;
use noisy_bevy::simplex_noise_3d;

use crate::sand::{Universe, UniverseConfig};
use crate::substances::Substance;

pub struct RenderPlugin;
impl bevy::app::Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin);
        app.add_plugins(PerfUiPlugin);
        app.add_systems(Startup, setup);
        app.add_systems(Update, draw);
    }
}

#[derive(Resource)]
struct OutputImage(Handle<Image>);

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>, config: Res<UniverseConfig>) {
    commands.spawn(PerfUiFramerateEntries::default());
    commands.spawn(Camera2d::default());

    // create an image that we are going to draw into
    let image = Image::new_fill(
        // 2D image of size 256x256
        Extent3d {
            width: config.width.try_into().expect("Universe width doesn't fit into i32!"),
            height: config.height.try_into().expect("Universe height doesn't fit into i32!"),
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        // use red for the default color so we can tell if something is wrong
        &(css::RED.to_u8_array()),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    let handle = images.add(image);
   

    commands.spawn((
        Sprite::from_image(handle.clone()),
        Transform::from_scale(Vec3::new(config.scale, config.scale, config.scale))
    ));
    commands.insert_resource(OutputImage(handle));
}

fn draw(handle: Res<OutputImage>, mut images: ResMut<Assets<Image>>, universe: Res<Universe>, time: Res<Time>) {
    let image = images.get_mut(&handle.0).expect("Image not found");
    for x in 0..universe.width {
        for y in 0..universe.height {
            let color = get_color(x, y, time.elapsed_secs(), universe.get(x, y).substance);
            let _ = image.set_color_at(
                x.try_into().unwrap(),
                y.try_into().unwrap(),
                color);
        }
    }
}

fn get_color(x: isize, y: isize, time: f32, substance: Substance) -> Color {
    match substance {
        // dynamic noise
        Substance::Water | Substance::Mud(..) => {
            let noise = simplex_noise_3d(
                Vec3 { 
                    x: x as f32,
                    y: y as f32,
                    z: time
                });
            let base_color = substance.default_color();
            return base_color.lighter(noise * 0.03)
        },
        // static noise
        Substance::Sand(..) | Substance::Rock | Substance::Dirt(..) | Substance::Grass(..) => {
            let noise = simplex_noise_3d(
                Vec3 { 
                    x: x as f32,
                    y: y as f32,
                    z: 1.0
                });
            let base_color = substance.default_color();
            return base_color.lighter(noise * 0.03)
        },
        _ => { return substance.default_color(); } 
    }
}
