use bevy::prelude::*;
use bevy::input::common_conditions::input_pressed;
use bevy::window::PrimaryWindow;
use bevy::color::{palettes::css, Color};

use crate::substances::*;

pub struct SandPlugin;
impl bevy::app::Plugin for SandPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UniverseConfig {
            width: 256,
            height: 256,
            scale: 2.0
        });
        app.insert_resource(SelectedSubstance(Substance::Void));
        app.add_systems(Startup, setup);
        app.add_systems(FixedUpdate, update_universe);
        app.add_systems(Update, select_substance);
        app.add_systems(Update, paint_substance.run_if(input_pressed(MouseButton::Left)));
    }
}

#[derive(Resource)]
pub struct SelectedSubstance(Substance);

#[derive(Resource)]
pub struct UniverseConfig {
    pub width: isize,
    pub height: isize,
    pub scale: f32,
}

#[derive(Default, Clone)]
pub struct Cell {
    pub substance: Substance,
    pub color: Color,
    pub has_ticked: bool
}

#[derive(Resource)]
pub struct Universe {
    vec: Vec<Cell>,
    pub width: isize,
    pub height: isize
}

impl Universe {
    pub fn with_dimensions(w: isize, h: isize) -> Universe {
        let mut universe = Universe {
            vec: Vec::<Cell>::with_capacity((w*h).try_into().unwrap()),
            width: w,
            height: h
        };

        universe.vec.resize(
            (w*h).try_into().unwrap(),
            Cell {
                substance: Substance::Void,
                color: Color::Srgba(css::BLACK),
                has_ticked: false
            });

        return universe;
    }

    pub fn get(&self, x: isize, y: isize) -> Cell {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return Cell {
                substance: Substance::OutOfBounds,
                ..default()
            };
        }
        let index = y*self.width + x;
        return self.vec[usize::try_from(index).ok().unwrap()].clone();
    }

    pub fn get_mut(&mut self, x: isize, y: isize) -> &mut Cell {
        let index = y*self.width + x;
        return &mut self.vec[usize::try_from(index).unwrap_or_else(|_| panic!("Invalid index for get_mut {x}, {y}"))];
    }

    pub fn swap(&mut self, x1: isize, y1: isize, x2: isize, y2: isize) {
        self.vec.swap(
            usize::try_from(y1*self.width + x1).unwrap_or_else(|_| panic!("Invalid index for swap {x1}, {y1}")),
            usize::try_from(y2*self.width + x2).unwrap_or_else(|_| panic!("Invalid index for swap {x2}, {y2}")))
    }
}

pub struct UniverseInterface<'a> {
    universe_ref: &'a mut Universe,
    pub x: isize,
    pub y: isize,
}

impl UniverseInterface<'_> {
    // get and set cells with offsets
    pub fn get(&self, x: isize, y: isize) -> Cell {
        let tx = self.x + x;
        let ty = self.y + y;
        return self.universe_ref.get(tx, ty);
    }

    pub fn set(&mut self, x: isize, y: isize, substance: Substance) {
        let tx = self.x + x;
        let ty = self.y + y;
        let target = self.universe_ref.get_mut(tx, ty);
        target.substance = substance.clone();
        target.has_ticked = true;
    }

    // swap the tile this interface is for with the given tile by relative coords
    pub fn swap(&mut self, x: isize, y: isize) {
        let tx = self.x + x;
        let ty = self.y + y;
        self.universe_ref.swap(self.x, self.y, tx, ty);
        self.universe_ref.get_mut(self.x, self.y).has_ticked = true;
        self.universe_ref.get_mut(tx, ty).has_ticked = true;
    }
}

fn setup(mut commands: Commands, config: Res<UniverseConfig>) {
    let universe = Universe::with_dimensions(config.width, config.height);
    println!("SandPlugin setting up...");
    println!("Universe size: {} x {}", universe.width, universe.height);
    commands.insert_resource(universe);
}

fn update_universe(mut universe: ResMut<Universe>) {
    // untick all cells
    for x in 0..universe.width {
        for y in 0..universe.height {
            universe.get_mut(x, y).has_ticked = false;
        }
    }

    // update all cells
    for x in 0..universe.width {
        for y in 0..universe.height {
            update_cell(UniverseInterface { universe_ref: &mut universe, x, y });
        }
    }
}

#[allow(unreachable_patterns)]
fn update_cell(interface: UniverseInterface) {
    let cell = interface.get(0, 0);
    if !cell.has_ticked {
        match cell.substance {
            Substance::Void => update_void(interface),
            Substance::Sand(..) => update_sand(interface),
            Substance::Rock => update_rock(interface),
            Substance::Water => update_water(interface),
            Substance::Dirt(..) => update_dirt(interface),
            Substance::Mud(..) => update_mud(interface),
            Substance::Grass(..) => update_grass(interface),
            other => {println!("updating {other}!");}
        }
    }
}

fn select_substance(mut select: ResMut<SelectedSubstance>, keys: Res<ButtonInput<KeyCode>>) {
    for key in keys.get_pressed() {
        select.0 = match key {
            KeyCode::Digit0 => Substance::Void,
            KeyCode::Digit1 => Substance::Rock,
            KeyCode::Digit2 => Substance::Sand(false),
            KeyCode::Digit3 => Substance::Water,
            KeyCode::Digit4 => Substance::Dirt(false, 0),
            KeyCode::Digit5 => Substance::Mud(false, 0),
            KeyCode::Digit6=> Substance::Grass(0),
            _ => select.0.clone()
        }
    }
}

fn paint_substance(
    mut universe: ResMut<Universe>,
    config: Res<UniverseConfig>,
    substance: Res<SelectedSubstance>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor)))
        .map(|ray| ray.unwrap().origin.truncate())
    {
        let universe_x = (world_position.x.round() as isize + universe.width)/config.scale as isize;
        let universe_y = (-world_position.y.round() as isize + universe.height)/config.scale as isize;
        if universe_x < 0 || universe_x >= universe.width || universe_y < 0 || universe_y >= universe.height {
            return
        }
        universe.get_mut(universe_x, universe_y).substance = substance.0.clone();
    }
}
