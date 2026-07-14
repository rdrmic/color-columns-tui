use std::path::PathBuf;

use crate::{errors, terminal};

pub fn get_app_data_dir_path() -> Option<PathBuf> {
    match create_data_dir() {
        Ok(path) => Some(path),
        Err(e) => {
            eprint!(
                "\nWARNING!\n\nFailed to create or access application data directory: '{e}'.\nHighscores and logs will not be saved!\n\nPress any key to proceed... "
            );
            terminal::get_key_press();
            eprintln!();

            None
        }
    }
}

fn create_data_dir() -> Result<PathBuf, errors::Error> {
    let mut dir_path = create_target_os_data_dir_path()?;
    dir_path.push(env!("CARGO_PKG_NAME"));

    std::fs::create_dir_all(&dir_path)?;

    Ok(dir_path)
}

#[cfg(target_os = "windows")]
fn create_target_os_data_dir_path() -> Result<PathBuf, errors::Error> {
    let dir_path = get_root_path("LOCALAPPDATA")?;
    Ok(dir_path)
}

#[cfg(target_os = "macos")]
fn create_target_os_data_dir_path() -> Result<PathBuf, errors::Error> {
    let mut dir_path = get_root_path("HOME")?;
    dir_path.push("Library/Application Support");
    Ok(dir_path)
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn create_target_os_data_dir_path() -> Result<PathBuf, errors::Error> {
    get_root_path("XDG_STATE_HOME").or_else(|_| {
        let mut dir_path = get_root_path("HOME")?;
        dir_path.push(".local/state");
        Ok(dir_path)
    })
}

#[allow(clippy::inline_always)]
#[inline(always)]
fn get_root_path(env_var: &str) -> Result<PathBuf, errors::Error> {
    let path = std::env::var_os(env_var).map(PathBuf::from).ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))?;
    Ok(path)
}
