use std::process::Command;

fn main() {
    // Run the Tailwind CSS build process
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
