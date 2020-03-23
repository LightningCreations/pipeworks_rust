extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main(){
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=libpipeworks/include/pipeworks/engine.h");
    println!("cargo:rerun-if-changed=libpipeworks/include/pipeworks/game.h");
    println!("cargo:rerun-if-changed=libpipeworks/include/pipeworks/thing.h");
    let builder = bindgen::builder()
        .header("bindings.h")
        .whitelist_type("pw_[A-Za-z_]+")
        .whitelist_function("pw_[A-Za-z_]+")
        .whitelist_var("STATE_PRIME")
        .no_copy("pw_[A-Za-z_]+")
        .generate()
        .expect("Unable to generate bindings");

    builder.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
}