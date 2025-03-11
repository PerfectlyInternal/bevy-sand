use std::fmt;
use bevy::prelude::*;
use bevy::color::{palettes::css, Color};
use grid::*;

use crate::substances::*;

pub struct SandPlugin;
impl bevy::app::Plugin for SandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(FixedUpdate, update_universe);
    }
}

#[derive(Default, Clone)]
pub enum Substance {
    #[default]
    Void,
    Sand,
    Rock,
    Water,
}

impl fmt::Display for Substance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           Substance::Void => write!(f, "void"),
           Substance::Sand => write!(f, "sand"),
           Substance::Rock => write!(f, "rock"),
           Substance::Water => write!(f, "water"),
       }
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
    pub grid: Grid<Cell>,
}

pub struct UniverseInterface<'a> {
    universe_ref: &'a mut Universe,
    pub x: isize,
    pub y: isize,
}

impl UniverseInterface<'_> {
    // get and set cells with offsets
    pub fn get(&self, x: isize, y: isize) -> Cell {
        return self.universe_ref.grid.get(y, x)
            .expect("Tried to access out-of-range cell at {x}, {y}!")
            .clone();
    }

    pub fn set(&mut self, x: isize, y: isize, substance: Substance, color: Color) {
        let grid = &mut self.universe_ref.grid;
        let target = grid.get_mut(self.y + y, self.x + x)
            .expect("Failed to get cell {self.x} + {x}, {self.y} + {y}!");
        target.substance = substance;
        target.color = color;
        target.has_ticked = true;
    }
}

fn setup(mut commands: Commands) {
    let mut universe = Universe { grid: Grid::new(256, 256) };
    universe.grid.fill(Cell { substance: Substance::Void, color: Color::Srgba(css::BLACK), has_ticked: false });
    println!("SandPlugin setting up...");
    println!("Universe size: {} x {}", universe.grid.cols(), universe.grid.rows());
    commands.insert_resource(universe);
}

fn update_universe(mut universe: ResMut<Universe>) {
    // untick all cells
    for x in 0..universe.grid.cols() {
        for y in 0..universe.grid.rows() {
            universe.grid.get_mut(y, x).unwrap().has_ticked = false;
        }
    }

    // update all cells
    for x in 0..universe.grid.cols() {
        for y in 0..universe.grid.rows() {
            update_cell(
                UniverseInterface{
                    universe_ref: &mut universe,
                    x: isize::try_from(x).ok().expect("Failed to convert {x} to isize. Is the Universe too big?"),
                    y: isize::try_from(y).ok().expect("Failed to convert {y} to isize. Is the Universe too big?")
                });
        }
    }

    // spawn some sand for fun
    for x in 10..20 {
        for y in 10..20 {
            universe.grid.get_mut(y, x).unwrap().substance = Substance::Sand;
            universe.grid.get_mut(y, x).unwrap().color = Color::Srgba(css::BEIGE);
        }
    }
}

fn update_cell(interface: UniverseInterface) {
    let cell = interface.get(0, 0);
    if !cell.has_ticked {
        match cell.substance {
            Substance::Void => update_void(interface),
            Substance::Sand => update_sand(interface),
            Substance::Rock => update_rock(interface),
            Substance::Water => update_water(interface),
        }
    }
}
