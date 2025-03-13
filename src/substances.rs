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
    Sand,
    Rock,
    Water,
}

impl fmt::Display for Substance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           Substance::Void => write!(f, "void"),
           Substance::OutOfBounds => write!(f, "oob"),
           Substance::Sand => write!(f, "sand"),
           Substance::Rock => write!(f, "rock"),
           Substance::Water => write!(f, "water"),
       }
    }
}

impl Substance {
    #[allow(unreachable_patterns)]
    pub fn default_color(&self) -> Color {
        return Color::Srgba(match self {
            Substance::Void => css::BLACK,
            Substance::Sand => css::BEIGE,
            Substance::Rock => css::GREY,
            Substance::Water => css::BLUE,
            _ => css::RED
        });
    }
}

pub fn update_void(mut _interface: UniverseInterface) {

}

pub fn update_sand(mut interface: UniverseInterface) {
    let offset = rand::thread_rng().gen_range(-1..2);
    match interface.get(offset, 1).substance {
        Substance::Void => {
            let sand = interface.get(0, 0).substance;
            let color = interface.get(0, 0).color;
            interface.set(offset, 1, sand, color);
            interface.set(0, 0, Substance::Void, Substance::Void.default_color());
        },
        _ => {}
    }
}

pub fn update_rock(mut _interface: UniverseInterface) {

}

pub fn update_water(mut interface: UniverseInterface) {
    let offsetx = rand::thread_rng().gen_range(-1..2);
    let offsety = rand::thread_rng().gen_range(0..2);
    match interface.get(offsetx, offsety).substance {
        Substance::Void => {
            let water = interface.get(0, 0).substance;
            let color = interface.get(0, 0).color;
            interface.set(offsetx, offsety, water, color);
            interface.set(0, 0, Substance::Void, Substance::Void.default_color());
        },
        _ => {}
    }
}
