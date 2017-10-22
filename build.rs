use std::io;
use std::io::Write;
use std::process;
use std::process::Command;
use std::str;

fn main() {
    match build_blend() {
        Ok(()) => (),
        Err(error) => {
            writeln!(io::stderr(), "{}", error).unwrap();
            process::exit(1);
        },
    }
}

fn build_blend() -> Result<(), String> {
    let output = Command::new("tools/build_assets.py").output().map_err(|e| e.to_string())?;
    if output.status.success() {
        Ok(())
    } else {
        Err(str::from_utf8(&output.stderr).unwrap_or("").to_string())
    }
}
