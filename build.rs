extern crate datetime;

fn main() {
    println!(
        "cargo:rustc-env=GIT_HASH={}",
        rustc_tools_util::get_commit_hash().unwrap_or_default()
    );
    println!("cargo:rustc-env=BUILD_DATE={}", build_date());
}

fn build_date() -> String {
    use datetime::{LocalDateTime, ISO};

    let now = LocalDateTime::now();
    format!("{}", now.date().iso())
}
