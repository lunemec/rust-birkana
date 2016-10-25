extern crate svg;

use std::io::{self, Read, Write};

use svg::Document;
use svg::node::element::Path;

mod letters;


fn create_path(data: svg::node::element::path::Data) -> svg::node::element::Path {
    let path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 1)
                .set("d", data);
    return path;
}

fn create_document_from_string(input: String, spacing: i32) -> svg::node::element::SVG {
    let mut position = 0;
    let mut document = Document::new();

    for character in String::from(input).to_uppercase().chars() {
        let data = match character {
            '0' => letters::create_0_data(spacing, position),
            '1' => letters::create_1_data(spacing, position),
            '2' => letters::create_2_data(spacing, position),
            '3' => letters::create_3_data(spacing, position),
            '4' => letters::create_4_data(spacing, position),
            '5' => letters::create_5_data(spacing, position),
            '6' => letters::create_6_data(spacing, position),
            '7' => letters::create_7_data(spacing, position),
            '8' => letters::create_8_data(spacing, position),
            '9' => letters::create_9_data(spacing, position),
            'A' => letters::create_a_data(spacing, position),
            'B' => letters::create_b_data(spacing, position),
            'C' => letters::create_c_data(spacing, position),
            'D' => letters::create_d_data(spacing, position),
            'E' => letters::create_e_data(spacing, position),
            'F' => letters::create_f_data(spacing, position),
            _ => continue,
        };
        document = document.add(create_path(data));
        position += 1;
    }
    return document;
}

fn main() {
    let spacing = 40;
    let mut buffer = String::new();
    
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => {},
        Err(e) => {
            println!("error reading from stdin: {:?}", e);
            return;
        }
    }
    let document = create_document_from_string(buffer, spacing);
    
    match io::stdout().write(&document.to_string().into_bytes()) {
        Ok(_) => {},
        Err(e) => {
            println!("error writing to stdout: {:?}", e);
            return;
        },
    }
}
