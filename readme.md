# rpfixer

This tool allows you to unfuck corrupted images metadata in a resource pack that have been corrupted to be protected.

This tool will not overcome any protection to the zip archive itself. See a program such as [MCRPX](https://github.com/Speedy11CZ/mcrpx) for that.

# Usage


## CLI

### Manual clone
- download the Rust-Lang for your system
- download this repository
- run `cargo build --release` in the directory
- check `./target/release`
- run 
  - `respack-decrypter` if you are on linux/mac (RUN `chmod +x respack-decrypter` ON IT FIRST) 
  - `respack-decrypter.exe` if you are on windows
### Cargo
- Download the Rust-Lang for your system
- run `cargo install rpfixer`
- Then run the `rpfixer` command


# Syntax

Check `--help` for syntax.

# I want some logs for checking wtf is happening

Pipe your output to a new filename. 

Note: It might not work for SOME linux shells. But major ones e.g. `zsh`,`bash` support it.

`respack-decrypter --path ~/somedir/ --debug > ~/log.txt`

# License

It's all MIT, except the example corrupted inventory.png which is courtesy of Wynncraft. 

Zeer you better not come after me for this one, you still haven't answered my gdpr req yet