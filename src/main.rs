use std::env;
fn main() {
    // Manière simple
    println!(
        "Version depuis Cargo.toml(CARGO_PKG_VERSION) : {}",
        env::var("CARGO_PKG_VERSION").unwrap()
    );

    // Via « Build script »
    println!(
        "Version depuis build.rs(MYPROJECT_VERSION) : {}",
        env::var("MYPROJECT_VERSION").unwrap()
    );
}
