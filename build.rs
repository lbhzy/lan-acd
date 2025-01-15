use chrono::Local;

fn main() {
    let now = Local::now();

    println!("cargo:rustc-env=BUILD_DATE={}", now.format("%Y-%m-%d %H:%M:%S %Z"));
}