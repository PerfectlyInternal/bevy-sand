use bevy::prelude::*;
use bevy::color::{palettes::css, Color};

use crate::substances::*;

pub struct SandPlugin;
impl bevy::app::Plugin for SandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(FixedUpdate, update_universe);
    }
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
        return &mut self.vec[usize::try_from(index).ok().unwrap()];
    }

    pub fn swap(&mut self, x1: isize, y1: isize, x2: isize, y2: isize) {
        self.vec.swap(
            usize::try_from(y1*self.width + x1).ok().unwrap(),
            usize::try_from(y2*self.width + x2).ok().unwrap());
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
        self.universe_ref.swap(self.x, self.y, self.x + x, self.y + y);
    }
}

fn setup(mut commands: Commands) {
    let mut universe = Universe::with_dimensions(256, 256);
    for x in 10..250 {
        for y in 5..25 {
            universe.get_mut(x, y).substance = Substance::Water;
            universe.get_mut(x, y).color = Color::Srgba(css::BLUE);
        }
    }
    for x in 10..250 {
        for y in 25..50 {
            universe.get_mut(x, y).substance = Substance::Rock;
            universe.get_mut(x, y).color = Color::Srgba(css::GREY);
        }
    }
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
            Substance::Sand(_) => update_sand(interface),
            Substance::Rock => update_rock(interface),
            Substance::Water => update_water(interface),
            Substance::Dirt => update_dirt(interface),
            other => {println!("updating {other}!");}
        }
    }
}
