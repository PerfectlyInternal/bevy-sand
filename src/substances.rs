use bevy::color::{palettes::css, Color};

use crate::sand::{UniverseInterface, Substance};

pub fn update_void(mut interface: UniverseInterface) {
    interface.set(0, 0, Substance::Void, Color::Srgba(css::BLACK));
}

pub fn update_sand(mut interface: UniverseInterface) {
    match interface.get(0, 1).substance {
        Substance::Void => {
            let sand = interface.get(0, 0).substance;
            let color = interface.get(0, 0).color;
            interface.set(0, 1, sand, color);
            interface.set(0, 0, Substance::Void, Color::Srgba(css::BLACK));
            println!("some sand just fell!");
        },
        Substance::Sand => {}
        other => {println!("some sand is beside some {other}")}
    }
}

pub fn update_rock(mut interface: UniverseInterface) {
    interface.set(0, 0, Substance::Rock, Color::Srgba(css::GRAY));
}

pub fn update_water(mut interface: UniverseInterface) {
    for i in -1..2 {
        if let Substance::Void = interface.get(i, 1).substance {
            let water = interface.get(0, 0).substance;
            let color = interface.get(0, 0).color;
            interface.set(0, 1, water, color);
            interface.set(0, 0, Substance::Void, Color::Srgba(css::BLACK));
        }
    }
}
