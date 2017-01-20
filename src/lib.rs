extern crate svg;

use svg::Document;

pub mod rune;
use rune::{Rune, RuneData};


pub fn document_from_string(input: String) -> svg::node::element::SVG {
    let mut position = 0;
    let mut document = Document::new();

    for character in input.to_uppercase().chars() {
        let mut x_start = 0;
        // First letter will start 5 units from the left image border.
        if position == 0 {
            x_start = 10;
        } 
        let path = match character {
            '0' => Rune::from_data(RuneData::new(position, x_start).x0().close()).to_path(),
            '1' => Rune::from_data(RuneData::new(position, x_start).x0().x1().close()).to_path(),
            '2' => Rune::from_data(RuneData::new(position, x_start).x0().x2().close()).to_path(),
            '3' => Rune::from_data(RuneData::new(position, x_start).x0().x1().x2().close()).to_path(),
            '4' => Rune::from_data(RuneData::new(position, x_start).x0().x4().close()).to_path(),
            '5' => Rune::from_data(RuneData::new(position, x_start).x0().x1().x4().close()).to_path(),
            '6' => Rune::from_data(RuneData::new(position, x_start).x0().x2().x4().close()).to_path(),
            '7' => Rune::from_data(RuneData::new(position, x_start).x0().x1().x2().x4().close()).to_path(),
            '8' => Rune::from_data(RuneData::new(position, x_start).x0().x8().close()).to_path(),
            '9' => Rune::from_data(RuneData::new(position, x_start).x0().x1().x8().close()).to_path(),
            'A' => Rune::from_data(RuneData::new(position, x_start).x0().x2().x8().close()).to_path(),
            'B' => Rune::from_data(RuneData::new(position, x_start).x0().x1().x2().x8().close()).to_path(),
            'C' => Rune::from_data(RuneData::new(position, x_start).x0().x4().x8().close()).to_path(),
            'D' => Rune::from_data(RuneData::new(position, x_start).x0().x1().x4().x8().close()).to_path(),
            'E' => Rune::from_data(RuneData::new(position, x_start).x0().x2().x4().x8().close()).to_path(),
            'F' => Rune::from_data(RuneData::new(position, x_start).x0().x1().x2().x4().x8().close()).to_path(),
            _ => continue,
        };
        document = document.add(path);
        position += 1;
    }
    document = document.set("viewBox", (0, 0, 40 * input.chars().count(), 120));
    return document;
}