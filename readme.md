# rpfixer

This tool allows you to unfuck corrupted images metadata in a resource pack that have been corrupted to be protected.

This tool will not overcome any protection to the zip archive itself. See a program such as [MCRPX](https://github.com/Speedy11CZ/mcrpx) for that.

# Usage


## CLI

#### Manual clone
- download the Rust-Lang for your system
- download this repository
- run `cargo build --release` in the directory
- check `./target/release`
- run 
  - `respack-decrypter` if you are on linux/mac (RUN `chmod +x respack-decrypter` ON IT FIRST) 
  - `respack-decrypter.exe` if you are on windows
#### Cargo
- Download the Rust-Lang for your system
- run `cargo install rpfixer`
- Then run the `rpfixer` command

## Function library
- The library has one function: `rpfixer::idk::fix`. Use a `Vec<u8>` in it.
#### Example code
```rust
use std::fs;
fn main() {
    let frfr = fs::read("filepath.png").expect("wtf the path doesnt exist");
    let mut fr = rpfixer::idk::fix(frfr);
    fs::write("filepath.png", fr).expect("file could not write btw");
}
```


# Syntax

Check `--help` for syntax.

Example syntax:


# License

It's all MIT, except the example corrupted inventory.png which is courtesy of Wynncraft. 

Zeer you better not come after me for this one, you still haven't answered my gdpr req yet