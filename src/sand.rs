use bevy::prelude::*;
use grid::*;

pub struct SandPlugin;
impl bevy::app::Plugin for SandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Default)]
pub enum Substance {
    #[default]
    Void,
    Sand(u8),
    Rock,
    Water,
}

#[derive(Default)]
pub struct Cell {
    pub substance: Substance,
    pub color: Color,
    pub clock: u8
}

#[derive(Resource)]
pub struct Universe {
    pub grid: Grid<Cell>
}

fn setup(mut commands: Commands) {
    let universe = Universe { grid: Grid::with_capacity(256, 256) };
    print!("{}", universe.grid.cols());
    commands.insert_resource(universe);
}
