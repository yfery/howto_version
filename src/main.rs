use std::env;
fn main() {
    // Manière simple
    println!(
        "Depuis Cargo.toml(CARGO_PKG_VERSION) : {}",
        env::var("CARGO_PKG_VERSION").unwrap()
    );

    // Via « Build script »
    println!(
        "Avec les informations de build.rs(MYPROJECT_VERSION) : {}",
        format!(
            "v{} ({} built on {})",
            env::var("CARGO_PKG_VERSION").unwrap(),
            env::var("GIT_HASH").unwrap(),
            env::var("BUILD_DATE").unwrap(),
        )
    );
}
