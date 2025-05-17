use std::path::PathBuf;
use bindgen;

fn main() {
    let deps = vec!["user32", "psapi", "ntdll", "shell32"];
    for dep in deps {
        println!("cargo:rustc-link-lib={}", dep);
    }
    // JNIHook MT Release
    println!("cargo:rustc-link-search={}", "3rd/build_MD/Release");
    println!("cargo:rustc-link-lib=jnihook");
    // JDK used during build of JNIHook
    println!("cargo:rustc-link-search={}", "3rd/build_MD/_deps/openjdk-src/lib");
    println!("cargo:rustc-link-lib=jvm"); // Link against jvm.lib
    println!("cargo:rustc-link-lib=jawt"); // Link against jawt.lib

        // Configure and generate bindings.
    let bindings = bindgen::Builder::default()
        .clang_arg("-I3rd/build_MT/_deps/openjdk-src/include")
        .clang_arg("-I3rd/build_MT/_deps/openjdk-src/include/win32")
        .clang_arg("-std=c++2b")
        // The input header we would like to generate
        // bindings for.
        .header("3rd/include/wrapper.hpp")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the generated bindings to an output file.
    let out_path = PathBuf::from(".");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Configure and generate bindings.
    let bindings = bindgen::Builder::default()
        .clang_arg("-I3rd/build_MT/_deps/openjdk-src/include")
        .clang_arg("-I3rd/build_MT/_deps/openjdk-src/include/win32")
        // .clang_arg("-std=c++2b")
        // The input header we would like to generate
        // bindings for.
        .header("3rd/include/jnihook.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the generated bindings to an output file.
    let out_path = PathBuf::from(".");
    bindings
        .write_to_file(out_path.join("lite-bindings.rs"))
        .expect("Couldn't write bindings!");
}
