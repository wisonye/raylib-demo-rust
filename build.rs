///
/// Let the compiler to link the installed `raylib` libraries
///
/// MacOS: `brew install raylib`
///
fn main() {

    println!("cargo:rustc-link-lib=raylib"); // the "-l" flag
}
