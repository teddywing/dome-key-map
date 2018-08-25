extern crate cbindgen;

use cbindgen::Language;

fn main() {
    let crate_dir = env!("CARGO_MANIFEST_DIR");

    let config = cbindgen::Config::from_file("cbindgen.toml")
        .expect("Failed to read 'cbindgen.toml'");

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        // .with_language(Language::C)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("dome_key_map.h");
}
