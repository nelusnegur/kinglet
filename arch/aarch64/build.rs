//! This build script is used to ensure this crate is rebuilt
//! whenever there are changes to the `kinglet.ld` linker script.

fn main() {
    println!("cargo:rerun-if-changed=./src/kinglet.ld");
}
