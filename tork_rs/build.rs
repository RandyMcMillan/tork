use std::env;
use std::path::PathBuf;

fn main() {
    let ffi_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .join("ffi");

    // Link the Go shared library
    println!("cargo:rustc-link-search=native={}", ffi_dir.display());
    println!("cargo:rustc-link-lib=dylib=tork");

    // Set rpath so the binary can find libtork.so at runtime
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", ffi_dir.display());

    // Rebuild if the header changes
    let header = ffi_dir.join("libtork.h");
    println!("cargo:rerun-if-changed={}", header.display());

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header(header.to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_function("tork_.*")
        .allowlist_type("C(Task|Job|Node|User|Role|Mount|Metrics)")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
