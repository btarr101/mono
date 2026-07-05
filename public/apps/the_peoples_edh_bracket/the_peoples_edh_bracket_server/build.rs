use std::{path::Path, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=ALWAYS");
    if cfg!(debug_assertions) {
        return;
    }

    let client_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("parent")
        .join("the-peoples-edh-bracket-client");

    let output = Command::new("pnpm")
        .arg("build")
        .current_dir(client_dir)
        .output()
        .expect("failed to execute client build command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    for line in stdout.lines() {
        println!("cargo:warning=[stdout] {}", line);
    }
    for line in stderr.lines() {
        println!("cargo:warning=[stderr] {}", line);
    }

    assert!(output.status.success(), "client build command exited with an error status");
}
