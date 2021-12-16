fn main() {
    // Make sure changes of test files results in a rebuild so the macros are generated.
    println!("cargo:rerun-if-changed=tests/cases/")
}
