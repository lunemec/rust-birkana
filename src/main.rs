extern crate rust_birkana;

use std::io::{self, Read, Write};
use rust_birkana::document_from_string;


fn main() {
    let mut buffer = String::new();

    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => {}
        Err(e) => {
            println!("error reading from stdin: {:?}", e);
            return;
        }
    }
    let document = document_from_string(buffer);

    match io::stdout().write(&document.to_string().into_bytes()) {
        Ok(_) => {}
        Err(e) => {
            println!("error writing to stdout: {:?}", e);
            return;
        }
    }
}
