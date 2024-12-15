# rpfixer (formerly respack-decrypter)

This tool allows you to fix corrupted images' metadata in a resource pack, where the images have been corrupted to be protected.

This tool will not overcome any protection to the zip archive itself. See a program such as [MCRPX](https://github.com/Speedy11CZ/mcrpx) for that.

# Usage


## CLI

#### Manual clone
- download the Rust-Lang for your system
- download this repository
- run `cargo build --release` in the directory
- check `./target/release`
- run 
  - `rpfixer` if you are on linux/mac (RUN `chmod +x rpfixer` ON IT FIRST) 
  - `rpfixer` if you are on windows
#### Cargo
- Download the Rust-Lang for your system
- run `cargo install rpfixer`
- Then run the `rpfixer` command

## Function library
- The library has one function: `rpfixer::fix`. Use a `Vec<u8>` in it.
#### Example code
```rust
use std::fs;
fn main() {
    let frfr: Vec<u8> = fs::read("filepath.png").expect("wtf the path doesnt exist");
    let mut fr: Vec<u8> = rpfixer::fix(frfr);
    fs::write("filepath.png", fr).expect("file could not write btw");
}
```


# Syntax

Check `--help` for syntax.

Example syntax:


# License

It's all MIT, except the example corrupted inventory.png and inventory-fix.png which is courtesy of Wynncraft. 

Zeer you better not come after me for this one, you guys still haven't answered my gdpr req yet