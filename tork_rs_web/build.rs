use std::env;
use std::path::PathBuf;

fn main() {
    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    let ffi_dir = PathBuf::from(&manifest).join("..").join("ffi");
    let ffi_dir = ffi_dir.canonicalize().expect("ffi/ directory not found");

    // Link the shared library
    println!("cargo:rustc-link-search=native={}", ffi_dir.display());
    println!("cargo:rustc-link-lib=dylib=tork_web");

    // Set rpath so tests/bins find the .so at runtime
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", ffi_dir.display());

    // Re-run if the header changes
    let header = ffi_dir.join("libtork_web.h");
    println!("cargo:rerun-if-changed={}", header.display());

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header(header.to_string_lossy())
        .allowlist_function("tork_web_.*")
        .allowlist_function("tork_free_string")
        .allowlist_type("CJobSummary")
        .allowlist_type("CScheduledJob")
        .allowlist_type("CScheduledJobSummary")
        .allowlist_type("CQueueInfo")
        .allowlist_type("CHealthCheckResult")
        .allowlist_type("CTaskLogPart")
        .allowlist_type("CPage")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
