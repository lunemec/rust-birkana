# rust-birkana
Birkana SVG generator.

Recently I read [this awesome article](https://yawar.blogspot.cz/2016/10/the-birkana-hexadecimal-number-symbols.html) about representing hexadecimal numbers using runic system. So I created this little program that takes hexadecimal string on `stdin` and spits out SVG data on `stdout`.
The code is not that nice, I just wanted a working version.

## Usage:

    echo "0123456789abcdef" | rust-birkana > target.svg

This results in this kind of SVG image:

![birkana alphabet](https://lunemec.github.io/rust-birkana/svg/alphabet.svg)

This is a image of my name encoded in UTF-8 and represented as HEX:

<img src="https://lunemec.github.io/rust-birkana/svg/myname.svg" width="1000">
