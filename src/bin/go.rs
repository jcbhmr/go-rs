use std::{env, error::Error, io::Cursor, process::Command};
use cross_exec::CommandExt;

static ZIP: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/go.zip"));

fn main() -> Result<(), Box<dyn Error>> {
    let current_exe = env::current_exe()?;
    let data_dir = current_exe.with_added_extension("data");
    if !data_dir.try_exists()? {
        fs_err::create_dir_all(&data_dir)?;
        let mut archive = zip::ZipArchive::new(Cursor::new(ZIP))?;
        archive.extract(&data_dir)?;
    }
    Err(Command::new(data_dir.join("bin").join(format!("go{}", if cfg!(windows) { ".exe" } else { "" })))
        .args(env::args_os().skip(1))
        .cross_exec().into())
}
