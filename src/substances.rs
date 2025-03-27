use std::fmt;
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
    // it also grows every 5 random ticks, up to a max height of 5 cells
    // first u8 is number of random ticks, second is remaining height to grow
    Grass(u8, u8),
    // mud falls like sand, turns into dirt with enough exposure to air
    Mud(bool, u8),
    // fire spreads to nearby flammable cells
    // u8 is remaining fuel left to burn, 50% chance of consuming one each tick
    Fire(u8),
    // smoke spawns from fire, and decays into void after its time reaches 0
    // drifts upwards the same way water falls
    Smoke(u8),

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
           Substance::Grass(a, b) => write!(f, "grass, grow time: {a}, height remaining: {b}"),
           Substance::Fire(a) => write!(f, "fire, fuel: {a}"),
           Substance::Smoke(a) => write!(f, "smoke, time left: {a}"),
       }
    }
}

impl Substance {
    #[allow(unreachable_patterns)]
    pub fn default_color(&self) -> Color {
        Color::Srgba(match self {
            Substance::Void => css::BLACK,
            Substance::Sand(..) => css::BEIGE,
            Substance::Rock => css::GREY,
            Substance::Water => css::BLUE,
            Substance::Dirt(..) => css::SADDLE_BROWN,
            Substance::Mud(..) => css::BROWN,
            Substance::Grass(..) => css::GREEN,
            Substance::Fire(..) => css::GOLDENROD,
            Substance::Smoke(..) => css::DARK_GRAY,
            _ => css::RED
        })
    }
}

pub fn update_void(mut _interface: UniverseInterface) {
    // does nothing
}

pub fn update_sand(mut interface: UniverseInterface) {
    let mut offset = 0;
    if let Substance::Sand(false) = interface.get(0, 0).substance {
        offset = rand::thread_rng().gen_range(-1..2);
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
    if let Substance::Water = interface.get(offsetx, offsety).substance {
        if rand::thread_rng().gen_range(0..10) > 6 {
            interface.set(0, 0, Substance::Sand(false));
        }
    }
}

pub fn update_water(mut interface: UniverseInterface) {
    let offset = rand::thread_rng().gen_range(-1..2);
    if let Substance::Void = interface.get(offset, 1).substance {
        interface.swap(offset, 1);
        return;
    }
    let offset = rand::thread_rng().gen_range(-1..2);
    if let Substance::Void = interface.get(offset, 0).substance {
        interface.swap(offset, 0);
    }
}

pub fn update_dirt(mut interface: UniverseInterface) {
    let mut offset = 0;
    let mut expose_time = 0;
    if let Substance::Dirt(false, time) = interface.get(0, 0).substance {
        if time > 200 {
            interface.set(0, 0, Substance::Grass(0, 5));
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
    if let Substance::Grass(time, remheight) = interface.get(0, 0).substance {
        match interface.get(0, -1).substance {
            Substance::Void => {
                if remheight > 0 && time > 5 {
                    interface.set(0, -1, Substance::Grass(0, remheight - 1));
                // only tick the grass growth about 20% of the time, to make it more organic
                } else if rand::thread_rng().gen_range(0..10) > 8 {
                    interface.set(0, 0, Substance::Grass(time + 1, remheight));
                }
            },
            Substance::Grass(_, rh) => {
                if remheight > 0 && rh < remheight - 1 {
                    interface.set(0, 0, Substance::Grass(0, remheight - 1))
                }
            },
            Substance::Fire(..) => {}
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
        },
        _ => {
            if let Substance::Void = interface.get(0, -1).substance {
                expose_time += 1;
            }
            interface.set(0, 0, Substance::Mud(false, expose_time));
        }
    }

}

pub fn update_fire(mut interface: UniverseInterface) {
    let offsetx = rand::thread_rng().gen_range(-1..2);
    let offsety = rand::thread_rng().gen_range(-1..2);
    match interface.get(offsetx, offsety).substance {
        Substance::Grass(..) => {
            // ignite with 1 in 2 chance
            if rand::thread_rng().gen_range(0..10) > 5 {
                interface.set(offsetx, offsety, Substance::Fire(10));
            }
        }
        Substance::Void => {
            // spawn smoke with 1 in 5 chance
            if rand::thread_rng().gen_range(0..10) > 7 {
                let smoke_time = rand::thread_rng().gen_range(100..250);
                interface.set(offsetx, offsety, Substance::Smoke(smoke_time));
            }
        }
        _ => {}
    }
    if let Substance::Fire(fuel) = interface.get(0, 0).substance {
        if rand::thread_rng().gen_range(0..10) > 5 {
            if fuel < 1 {
                let smoke_time = rand::thread_rng().gen_range(100..250);
                interface.set(0, 0, Substance::Smoke(smoke_time));
            }
            else {
                interface.set(0, 0, Substance::Fire(fuel - 1));
            }
        }
    }
}

pub fn update_smoke(mut interface: UniverseInterface) {
    if let Substance::Smoke(time) = interface.get(0, 0).substance {
        if time > 0 {
            interface.set(0, 0, Substance::Smoke(time - 1));
        } else {
            interface.set(0, 0, Substance::Void);
            return;
        }
    }
    let offset = rand::thread_rng().gen_range(-1..2);
    if let Substance::Void = interface.get(offset, -1).substance {
        interface.swap(offset, -1);
        return;
    }
    let offset = rand::thread_rng().gen_range(-1..2);
    if let Substance::Void = interface.get(offset, 0).substance {
        interface.swap(offset, 0);
    }
}
