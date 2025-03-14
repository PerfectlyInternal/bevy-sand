use std::fmt;
use rand;
use rand::Rng;
use bevy::color::{palettes::css, Color};

use crate::sand::UniverseInterface;

#[allow(dead_code)]
#[derive(Default, Clone)]
pub enum Substance {
    #[default]
    Void,
    OutOfBounds,
    // bool represents sand falling state
    Sand(bool),
    Rock,
    Water,
    Dirt,
}

impl fmt::Display for Substance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           Substance::Void => write!(f, "void"),
           Substance::OutOfBounds => write!(f, "oob"),
           Substance::Sand(a) => write!(f, "sand, falling: {a}"),
           Substance::Rock => write!(f, "rock"),
           Substance::Water => write!(f, "water"),
           Substance::Dirt => write!(f, "dirt"),
       }
    }
}

impl Substance {
    #[allow(unreachable_patterns)]
    pub fn default_color(&self) -> Color {
        return Color::Srgba(match self {
            Substance::Void => css::BLACK,
            Substance::Sand(_) => css::BEIGE,
            Substance::Rock => css::GREY,
            Substance::Water => css::BLUE,
            Substance::Dirt => css::BROWN,
            _ => css::RED
        });
    }
}

pub fn update_void(mut _interface: UniverseInterface) {

}

pub fn update_sand(mut interface: UniverseInterface) {
    let mut offset = 0;
    match interface.get(0, 0).substance {
        Substance::Sand(false) => {
            offset = rand::thread_rng().gen_range(-1..2);
        },
        _ => {}
    }
    match interface.get(offset, 1).substance {
        Substance::Void | Substance::Water => {
            interface.set(0, 0, Substance::Sand(true));
            interface.swap(offset, 1);
        },
        _ => {
            interface.set(0, 0, Substance::Sand(false));
        }
    }
}

pub fn update_rock(mut interface: UniverseInterface) {
    let offsetx = rand::thread_rng().gen_range(-1..2);
    let offsety = rand::thread_rng().gen_range(-1..2);
    match interface.get(offsetx, offsety).substance {
        Substance::Water => {
            if rand::thread_rng().gen_range(0..10) > 6 {
                interface.set(0, 0, Substance::Sand(false));
            }
        },
        _ => {}
    }
}

pub fn update_water(mut interface: UniverseInterface) {
    let offset = rand::thread_rng().gen_range(-1..2);
    match interface.get(offset, 1).substance {
        Substance::Void => {
            interface.swap(offset, 1);
            return;
        },
        _ => {}
    }
    let offset = rand::thread_rng().gen_range(-1..2);
    match interface.get(offset, 0).substance {
        Substance::Void => {
            interface.swap(offset, 0);
        },
        _ => {}
    }
}

pub fn update_dirt(mut interface: UniverseInterface) {
    let mut offset = 0;
    match interface.get(0, 0).substance {
        Substance::Sand(false) => {
            offset = rand::thread_rng().gen_range(-1..2);
        },
        _ => {}
    }
    match interface.get(offset, 1).substance {
        Substance::Void | Substance::Water => {
            interface.set(0, 0, Substance::Sand(true));
            interface.swap(offset, 1);
        },
        _ => {
            interface.set(0, 0, Substance::Sand(false));
        }
    }
}
