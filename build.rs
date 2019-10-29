extern crate datetime;
use std::env;

fn main() {
    // Ajout du hash git dans les variables d'environnement
    let version: String;
    if let Some(rev) = git_revision_hash() {
        println!("cargo:rustc-env=BUILD_GIT_HASH={}", rev);

        version = format!(
            "v{} ({} built on {})",
            cargo_version(),
            rustc_tools_util::get_commit_hash().unwrap_or_default(),
            build_date()
        );
    } else {
        version = format!("v{} (built on {})", cargo_version(), build_date());
    }

    println!("cargo:rustc-env=MYPROJECT_VERSION={}", version);
}

fn cargo_version() -> String {
    env::var("CARGO_PKG_VERSION").unwrap()
}

fn git_revision_hash() -> Option<String> {
    let result = std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output();

    result.ok().and_then(|output| {
        let rev = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if rev.is_empty() {
            None
        } else {
            Some(rev)
        }
    })
}

fn build_date() -> String {
    use datetime::{LocalDateTime, ISO};

    let now = LocalDateTime::now();
    format!("{}", now.date().iso())
}
