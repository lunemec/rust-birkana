extern crate svg;

use std::io::{self, Read, Write};

use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

static TEXT: &'static str = "0123456789ABCDEF";

fn create_path(offset_increment: i32, position: i32) -> svg::node::element::Path {
    let x_offset = 10;
    let y_offset = 10;
    
    let data = Data::new()
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((0, 100))
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((25, 25))
                .move_to((x_offset + offset_increment * position + 25, y_offset + 25))
                .line_by((-25, 25))
                .move_to((x_offset + offset_increment * position, y_offset + 50))
                .line_by((25, 25))
                .move_to((x_offset + offset_increment * position + 25, y_offset + 75))
                .line_by((-25, 25))
                .close();

    let path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 1)
                .set("d", data);
    return path
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer);
    
    let spacing = 40;
    let mut document = Document::new();
    let mut position = 0;

    for character in String::from(TEXT).chars() {
        document = document.add(create_path(spacing, position));
        position += 1;
    }

    match io::stdout().write(&document.to_string().into_bytes()) {
        Ok(_) => {},
        Err(e) => {
            println!("error writing to stdout: {:?}", e);
            return
        },
    }
}
