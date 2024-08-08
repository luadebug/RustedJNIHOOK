fn main() {
    //println!("cargo:rustc-link-search=native=jnihook");
    let deps = vec!["user32", "psapi", "ntdll", "shell32"];
    for dep in deps {
        println!("cargo:rustc-link-lib={}", dep);
    }

    println!("cargo:rustc-link-search=native={}", "C:/Program Files/Eclipse Adoptium/jdk-20.0.2.9-hotspot/lib"); // Adjust the path to your JDK lib directory

    println!("cargo:rustc-link-lib=jvm"); // Link against jvm.lib
    println!("cargo:rustc-link-lib=jawt"); // Link against jawt.lib

    println!("cargo:rustc-link-lib=jnihook");
}