// create a build rs that will compile vite project when building the rust project in release mode

use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=package.json");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=public");

    if cfg!(debug_assertions) {
        return;
    }

    let output = Command::new("pnpm")
        .args(&["build"])
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
