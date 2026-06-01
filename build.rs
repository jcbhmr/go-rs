use std::{
    env,
    error::Error,
    io,
    process::{Command, Stdio},
};

fn main() -> Result<(), Box<dyn Error>> {
    let restore = true;
    // Go provides src/make.bash, src/make.bat, and src/make.rc to build Go. The
    // ".rc" scripts are for Plan 9. Rust does not have an officially supported
    // Plan 9 target, so we ignore it. That means our options are Unix-like bash
    // or Windows cmd.exe scripts.
    let make = if env::var("CARGO_CFG_WINDOWS").is_ok() {
        "./make.bat"
    } else {
        "./make.bash"
    };
    let status = Command::new(make)
        .current_dir("./go/src/")
        .args(&["-distpack"])
        .stdin(Stdio::null())
        .stdout(io::stderr())
        .stderr(io::stderr())
        .status()?;
    if !status.success() {
        return Err(format!("failed to build Go: {}", status).into());
    }

    Ok(())
}
