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
    // should only exist when trying to access an OOB cell
    OutOfBounds,
    // sand falls downwards
    // bool represents sand falling state
    Sand(bool),
    // rock stays in place, can be eroded by water
    Rock,
    // water falls, erodes rock, turns dirt into mud
    Water,
    // dirt falls like sand, and turns into mud when water touches it
    // dirt will turn into grass if exposed to air (or other grass) on top for long enough
    // the u8 value tracks this
    Dirt(bool, u8),
    // grass turns into dirt if something falls on it
    // it also grows every 200 ticks, up to a max height of 5 cells
    Grass(u8),
    // mud falls like sand, turns into dirt with enough exposure to air
    Mud(bool, u8),

}

impl fmt::Display for Substance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           Substance::Void => write!(f, "void"),
           Substance::OutOfBounds => write!(f, "oob"),
           Substance::Sand(a) => write!(f, "sand, falling: {a}"),
           Substance::Rock => write!(f, "rock"),
           Substance::Water => write!(f, "water"),
           Substance::Dirt(a, b) => write!(f, "dirt, falling: {a}, time exposed: {b}"),
           Substance::Mud(a, b) => write!(f, "mud, falling: {a} time exposed: {b}"),
           Substance::Grass(a) => write!(f, "grass, grow time: {a}"),
       }
    }
}

impl Substance {
    #[allow(unreachable_patterns)]
    pub fn default_color(&self) -> Color {
        return Color::Srgba(match self {
            Substance::Void => css::BLACK,
            Substance::Sand(..) => css::BEIGE,
            Substance::Rock => css::GREY,
            Substance::Water => css::BLUE,
            Substance::Dirt(..) => css::SADDLE_BROWN,
            Substance::Mud(..) => css::BROWN,
            Substance::Grass(..) => css::GREEN,
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
    let mut expose_time = 0;
    if let Substance::Dirt(false, time) = interface.get(0, 0).substance {
        if time > 200 {
            interface.set(0, 0, Substance::Grass(0));
            return
        }
        offset = rand::thread_rng().gen_range(-1..2);
        expose_time = time;
    }
    match interface.get(offset, 1).substance {
        Substance::Void => {
            interface.set(0, 0, Substance::Dirt(true, expose_time));
            interface.swap(offset, 1);
            return
        },
        _ => {
            if let Substance::Void = interface.get(0, -1).substance {
                expose_time += 1;
            }
            interface.set(0, 0, Substance::Dirt(false, expose_time));
        }
    }
    let offsetx = rand::thread_rng().gen_range(-1..2);
    let offsety = rand::thread_rng().gen_range(-1..2);
    if let Substance::Water = interface.get(offsetx, offsety).substance {
        interface.set(0, 0, Substance::Mud(false, 0));
        interface.set(offsetx, offsety, Substance::Void);
    }
}

pub fn update_grass(mut interface: UniverseInterface) {
    if let Substance::Grass(time) = interface.get(0, 0).substance {
        match interface.get(0, -1).substance {
            Substance::Void => {
                if time > 200 {
                    interface.set(0, -1, Substance::Grass(0));
                } else {
                    interface.set(0, 0, Substance::Grass(time + 1));
                }
            },
            Substance::Grass(..) => interface.set(0, 0, Substance::Grass(0)),
            _ => interface.set(0, 0, Substance::Dirt(false, 0)),
        }
    }
}

pub fn update_mud(mut interface: UniverseInterface) {
    let mut offset = 0;
    let mut expose_time = 0;
    if let Substance::Mud(false, time) = interface.get(0, 0).substance {
        if time > 200 {
            interface.set(0, 0, Substance::Dirt(false, 0));
            return
        }
        offset = rand::thread_rng().gen_range(-1..2);
        expose_time = time;
    }
    match interface.get(offset, 1).substance {
        Substance::Void | Substance::Water => {
            interface.set(0, 0, Substance::Mud(true, expose_time));
            interface.swap(offset, 1);
            return
        },
        _ => {
            if let Substance::Void = interface.get(0, -1).substance {
                expose_time += 1;
            }
            interface.set(0, 0, Substance::Mud(false, expose_time));
        }
    }

}
