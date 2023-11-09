use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=tailwind.config.js");
    println!("cargo:rerun-if-changed=src/styles/tailwind.css");
    println!("cargo:rerun-if-changed=src/resources/views/**/*.html");
    let status = Command::new("pnpm")
        .arg("run")
        .arg("build:css")
        .status()
        .expect("Failed to execute Tailwind CSS build process");

    if !status.success() {
        eprintln!("Error: Tailwind CSS build process failed");
        std::process::exit(1);
    }
}
