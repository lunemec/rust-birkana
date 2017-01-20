# rust-birkana
[![Crates.io]
(https://img.shields.io/crates/v/rust-birkana.svg)](https://crates.io/crates/rust-birkana)

Birkana SVG generator.

Recently I read [this awesome article](https://yawar.blogspot.cz/2016/10/the-birkana-hexadecimal-number-symbols.html) about representing hexadecimal numbers using runic system. So I created this little program that takes hexadecimal string on `stdin` and spits out SVG data on `stdout`.
The code is not that nice, I just wanted a working version.

## Download:
You can download binary versions for most common platforms from releases section.

## Build:
Since cross-compilation is not in very much working state right now, you'll have to build the binary yourselves.

Steps:
* **install rust**.

```
git clone https://github.com/lunemec/rust-birkana
cd rust-birkana
cargo build --release
mv target/release/rust-birkana .
```

Now you should be able use the program as described in `usage` section.

## Usage:

    echo "0123456789abcdef" | rust-birkana > target.svg

This results in this kind of SVG image:

<img src="https://lunemec.github.io/rust-birkana/svg/alphabet.svg" width="100%">

This is a image of my name encoded in UTF-8 and represented as HEX:

<img src="https://lunemec.github.io/rust-birkana/svg/myname.svg" width="100%">

## You can now use this crate as a module:
In your Cargo.toml add:

    [dependencies]
    rust-birkana = "1.1"

And use it like this:
```rust
extern crate rust_birkana;

use rust_birkana::document_from_string;


let document = document_from_string(my_string);
let svg_string = document.to_string();
```