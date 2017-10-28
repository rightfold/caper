use std::process::Command;

fn main() {
    build_blend();
}

fn build_blend() {
    let status = Command::new("tools/build_assets.py").status().unwrap();
    assert!(status.success());
}
